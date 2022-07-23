use serenity::{Client, model::{channel::{Message, Channel, self}, guild::Guild, gateway::Ready, id::ChannelId}, prelude::GatewayIntents, async_trait, client::{EventHandler, Context, Cache}, json::NULL, cache};
use std::{sync::Arc, env};

pub struct App {
    pub should_quit: bool,
    pub messages: Vec<Message>,
    pub channel: ChannelId,
}

impl App {
    pub fn new(id: ChannelId) -> Self {
        Self {
            should_quit: false,
            messages: Vec::new(),
            channel: id,
        }
    }
    
    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => self.should_quit = true,
            _ => {}
        }
    }
}
