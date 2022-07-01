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
    // let block = Block::default()
    //     .title("messages")
    //     .borders(Borders::ALL);
    // f.render_widget(block, chunks[1]);

    // keysmashes
    let msgs = vec![
        ListItem::new("asdf"),
        ListItem::new("qrg"),
        ListItem::new("g3rtgwfg"),
        ListItem::new("zcvrewgr"),
        ListItem::new("hyt3h"),
        ListItem::new("begbi"),
        ListItem::new(";wefj"),
        ListItem::new("qiwef"),
        ListItem::new("lshbv;"),
        ListItem::new("1923f1o"),
        ListItem::new("oijdfq"),
        ListItem::new("njfv;owief"),
        ListItem::new("vouwerug2394f"),
        ListItem::new("iwqnf;qwlef;jqwejfioqwefjqwbefqwhlefiqwhefliwqef"),
    ];
    let message_block = List::new(msgs)
        .block(Block::default().title("messages").borders(Borders::ALL));
        // .style(Style::default())
    f.render_widget(message_block, chunks[1]);
}
