use std::sync::{Arc, Mutex};

use serenity::{http::CacheHttp, prelude::Mentionable, futures::channel::oneshot::channel, model::channel::Channel};
use tui::{
    backend::Backend,
    widgets::{Block,List, Borders, ListItem},
    layout::{Constraint, Direction, Layout, Rect}, Frame
};

use super::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20)
            ]
            .as_ref()
        )
        .split(f.size());

    // message list
    let message_list: Vec<ListItem> = app.messages
        .iter()
        .map(|msg| ListItem::new(format!("{}: {}", msg.author.name, msg.content)))
        .collect::<Vec<ListItem>>();

    // make blocks
    let message_block = List::new(message_list)
        .block(Block::default()
        .title("I'll put the channel name here later")
        .borders(Borders::ALL));

    let text_input = Block::default()
        .title("Message")
        .borders(Borders::ALL);

    // render blocks
    f.render_widget(message_block, chunks[0]);
    f.render_widget(text_input, chunks[1]);
}
