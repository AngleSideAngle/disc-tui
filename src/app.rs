use serenity::{Client, model::{channel::{Message, Channel}, guild::Guild, gateway::Ready}, prelude::GatewayIntents, async_trait, client::{EventHandler, Context}, json::NULL};
use std::{sync::Arc, env};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _: Context, msg: Message) {
        println!("{}: {}", msg.author.name, msg.content);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub struct App {
    pub client: Client, // discord client
    messages: Vec<Message>,
    channels: Vec<Channel>,
    current_channel: Option<Channel>,
    current_guild: Option<Guild>,
}

impl App {
    pub async fn new(token: String) -> Self {
        // set up discord bot
        let intents = GatewayIntents::all();

        let mut bot = Client::builder(&token, intents)
            .event_handler(Handler)
            .await
            .expect("Err creating client");
        
        Self {
            client: bot,
            messages: Vec::new(),
            channels: Vec::new(),
            current_channel: Option::None,
            current_guild: Option::None,
        }
    }

    pub async fn start(&mut self) {
        if let Err(why) = &self.client.start().await {
            println!("Client error: {:?}", why);
        }
    }

    pub async fn get_guilds(&self) -> Vec<Guild>{
        let cache = &self.client.cache_and_http;
        cache.cache.guilds()
    }
}