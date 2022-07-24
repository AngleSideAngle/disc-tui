use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};

use serenity::{http::CacheHttp, prelude::Mentionable, futures::channel::oneshot::channel, model::channel::Channel};
use tui::{
    backend::Backend,
    widgets::{Block,List, Borders, ListItem, Paragraph},
    layout::{Constraint, Direction, Layout, Rect}, Frame, style::{Style, Color}
};

use crate::app::InputMode;

use super::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Max(20),
            Constraint::Percentage(80)
        ]
        .as_ref()
        )
        .split(f.size());
    
    draw_nav(f, app, chunks[0]);
    draw_channel(f, app, chunks[1]);
}

fn draw_nav<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let block = Block::default()
        .title("Nav")
        .borders(Borders::ALL);

    f.render_widget(block, area);
}

fn draw_channel<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Max(7)
            ]
            .as_ref()
        )
        .split(area);

    // update message height (-2 because of borders)
    app.height = chunks[0].height - 2;
    
    // generate message list
    let message_list: Vec<ListItem> = app.messages
        .iter()
        .map(|msg| ListItem::new(msg.to_string()))
        .collect();

    // make blocks
    let message_block = List::new(message_list)
        .block(Block::default()
        .title("Channel")
        .borders(Borders::ALL));

    let text_input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Viewing => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow)
        })
        .block(Block::default()
            .title("Message")
            .borders(Borders::ALL)
        );
        
    match app.input_mode {
        InputMode::Viewing => {},
        InputMode::Editing => {
            f.set_cursor(
                chunks[1].x + app.input.len() as u16 + 1,
                chunks[1].y + 1
            );
        }
    }

    // render blocks
    f.render_widget(message_block, chunks[0]);
    f.render_widget(text_input, chunks[1]);
}
