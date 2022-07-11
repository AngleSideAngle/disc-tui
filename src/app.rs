use serenity::{Client, model::{channel::{Message, Channel}, guild::Guild, gateway::Ready}, prelude::GatewayIntents, async_trait, client::{EventHandler, Context}, json::NULL};
use std::{sync::Arc, env};

pub struct App {
    pub messages: Vec<Message>,
    channels: Vec<Channel>,
    current_channel: Option<Channel>,
    current_guild: Option<Guild>,
}

impl App {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            channels: Vec::new(),
            current_channel: Option::None,
            current_guild: Option::None,
        }
    }

    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }
}
