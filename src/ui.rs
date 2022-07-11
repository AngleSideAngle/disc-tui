use std::sync::{Arc, Mutex};

use serenity::http::CacheHttp;
use tui::{
    backend::Backend,
    widgets::{Block,List, Borders, ListItem},
    layout::{Constraint, Direction, Layout, Rect}, Frame
};

use super::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: Arc<Mutex<App>>) {

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

    let state = app.lock().unwrap();
    let msgs: Vec<ListItem> = state.messages.iter().map(|msg|
        ListItem::new(format!("{}: {}", msg.author.name, msg.content)))
        .collect::<Vec<ListItem>>();
    
    let message_block = List::new(msgs)
        .block(Block::default().title("messages").borders(Borders::ALL));
        // .style(Style::default())
    f.render_widget(message_block, chunks[1]);
}
