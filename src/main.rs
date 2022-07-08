#![allow(unused_imports)]
mod app;
mod ui;

use std::cell::RefCell;
use std::io::Error;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, Sender};
use std::{env, thread, fmt};

use app::App;
use serenity::async_trait;
use serenity::futures::SinkExt;
use serenity::model::channel::{Message, Channel};
use serenity::model::error;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use tui::{Frame, terminal};
use tui::backend::Backend;
use tui::style::Style;
use tui::widgets::{List, ListItem};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use std::{
    io,
    time::{Duration, Instant},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct Handler {
    app: Arc<Mutex<App>>
}


#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _: Context, msg: Message) {
        let mut state = self.app.lock().unwrap();
        state.add_message(msg);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Arc::new(Mutex::new(App::new().await));
    
    // set up discord bot
    dotenv::dotenv().expect("failed to load .env file");
    let token = env::var("TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { app: Arc::clone(&app)})
        .await
        .expect("Err creating client");

    let state = Arc::clone(&app);
    thread::spawn(move || {
        start_ui(state).unwrap();
    });
    
    if let Err(why) = client.start().await {
        println!("serenity error: {:?}", why);
    }

    Ok(())
}

fn start_ui(app: Arc<Mutex<App>>) -> Result<(), Error> {
    // set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    loop {
        terminal.draw(|f| ui::draw(f, Arc::clone(&app)))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }
        }
    }

    // return terminal to original state
    disable_raw_mode()?;
    terminal.clear()?;
    terminal.show_cursor()?;

    Ok(())
}
