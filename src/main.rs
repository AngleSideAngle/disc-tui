#![allow(unused_imports)]
mod app;
mod ui;

use std::cell::RefCell;
use std::io::Error;
use std::process::exit;
use std::rc::Rc;
use std::sync::{Arc};
use std::sync::mpsc::{self, Receiver, Sender};
use std::{env, thread, fmt};

use app::{App, InputMode};
use serenity::futures::SinkExt;
use serenity::http::Http;
use serenity::model::channel::{Message, Channel};
use serenity::model::id::ChannelId;
use serenity::model::error;
use serenity::model::gateway::Ready;
use serenity::{prelude::*, async_trait, FutureExt};
use serenity::utils::{MessageBuilder, CustomMessage};
use tokio::time::{timeout, sleep};
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
use clap::Parser;

#[derive(Parser)]
#[clap(author = "Asa Paparo", version = "0.1", about)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
    Guilds,
    Channels {
        guild_id: u64
    },
    Open {
        channel_id: u64
    },
}

struct State;

impl TypeMapKey for State {
    type Value = Arc<Mutex<App>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let state = {
            let data_read = ctx.data.read().await;
            data_read.get::<State>().unwrap().clone()
        };
        
        {
            let mut state = state.lock().await;
            if state.channel == msg.channel_id {
                state.add_message(msg);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    // get token from env
    dotenv::dotenv().expect("failed to load .env file");
    let token = env::var("TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&token);
    
    match &args.action {
        Action::Guilds => {
            let guilds = http.get_guilds(None, None).await.unwrap();
            for guild in guilds {
                println!("{}: {}", guild.name, guild.id);
            }
            Ok(())
        },
        Action::Channels { guild_id } => {
            let guild = http.get_guild(guild_id.to_owned()).await.unwrap();
            for (id, channel) in guild.channels(http).await.unwrap() {
                println!("{} | {}: {}", channel.kind.name(), channel.name(), id.as_u64());
            }
            Ok(())
        },
        Action::Open { channel_id }=> {
            run(token, http, channel_id.to_owned()).await
        }
    }
}

/// runs the tui when a channel is opened
async fn run(token: String, http: Http, id: u64) -> Result<(), Error> {
    // app represents the application's state
    let app = Arc::new(Mutex::new(App::new(http, ChannelId(id))));

    let tick_rate = Duration::from_millis(100);

    // set up discord bot
    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .type_map_insert::<State>(Arc::clone(&app))
        .await
        .expect("Err creating client");
        
    let ui_res = tokio::spawn(async move {
        start_ui(app, tick_rate).await.unwrap();
    });
    let _discord_res = client.start().await;
    ui_res.await.unwrap();
    
    Ok(())
}

async fn start_ui(app: Arc<Mutex<App>>, tick_rate: Duration) -> Result<(), Error> {
    // set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // rendering loop
    loop {
        {
            let mut app = app.lock().await;

            terminal.draw(|f| ui::draw(f, &mut app))?;
            
            if crossterm::event::poll(tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    app.on_key(key).await;
                }
            }

            if app.should_quit {
                break;
            }
        }
    }

    // return terminal to original state
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    exit(0);
    // Ok(())
}
