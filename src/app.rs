use serenity::{Client, model::{channel::{Message, Channel}, guild::Guild, gateway::Ready}, prelude::GatewayIntents, async_trait, client::{EventHandler, Context, Cache}, json::NULL, cache};
use std::{sync::Arc, env};

pub struct App {
    pub messages: Vec<Message>,
    pub channels: Vec<Channel>,
    target_channel: Option<Channel>,
    target_guild: Option<Guild>,
}

impl App {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            channels: Vec::new(),
            target_channel: Option::None,
            target_guild: Option::None,
        }
    }
    
    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    // pub fn set_guild(&self, guild: Guild) {
    //     self.current_guild = guild;
    //     self.channels = 
    // }
}
