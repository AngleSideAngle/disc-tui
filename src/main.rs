use std::io::Error;
use std::{env, thread};

use serenity::async_trait;
use serenity::model::channel::{Message, Channel};
use serenity::model::error;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use tui::Frame;
use tui::backend::Backend;
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

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{}: {}", msg.author.name, msg.content);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // set up discord bot
    dotenv::dotenv().expect("failed to load .env file");
    let token = env::var("TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::all();

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let messages: Vec<ListItem> = Vec::new();
    let li = List::new(messages);
    
    let res = thread::spawn(|| {
        run()
    });
    
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    Ok(())
}

fn run() -> Result<(), io::Error>{
    // set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // return terminal to original state
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(70)
            ]
            .as_ref()
        )
        .split(f.size());

    let block = Block::default()
        .title("nav")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default()
        .title("messages")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}
