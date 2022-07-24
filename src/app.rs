use crossterm::event::{KeyEvent, KeyCode};
use serenity::{Client, model::{channel::{Message, Channel, self}, guild::Guild, gateway::Ready, id::{ChannelId, MessageId}}, prelude::GatewayIntents, async_trait, client::{EventHandler, Context, Cache}, json::NULL, cache, CacheAndHttp, http::Http};
use std::{sync::Arc, env, process::exit};

pub enum InputMode {
    Viewing,
    Editing,
}

pub struct App {
    http: Http,
    pub should_quit: bool,
    pub input_mode: InputMode,
    pub input: String,
    pub messages: Vec<String>,
    pub channel: ChannelId,
    pub height: u16,
}

impl App {
    pub fn new(http: Http, channel: ChannelId) -> Self {
        Self {
            http,
            should_quit: false,
            input_mode: InputMode::Viewing,
            input: String::new(),
            messages: Vec::new(),
            channel,
            height: 0,
        }
    }
    
    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(format!("{}: {}", msg.author.name, msg.content));
        // trim extra messages
        if self.messages.len() > self.height.into() {
            self.messages.drain(0..self.messages.len() - self.height as usize);
        }
    }

    async fn send_message(&mut self) {
        let res = self.channel.say(&self.http, &self.input).await;
        self.input.clear();
        if let Err(why) = res {
            self.messages.push(format!("[Error] {}", why.to_string()));
        }
    }

    pub async fn on_key(&mut self, key: KeyEvent) {
        match self.input_mode {
            InputMode::Viewing => match key.code {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                },
                KeyCode::Char('e') => {
                    self.input_mode = InputMode::Editing;
                },
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Viewing;
                },
                KeyCode::Char(c) => {
                    self.input.push(c);
                },
                KeyCode::Backspace => {
                    self.input.pop();
                },
                KeyCode::Enter => {
                    self.send_message().await;
                },
                _ => {}
            },
        }
    }
}
