use crate::app::App;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};
use ratatui::Frame;

pub fn render_library(app: &mut App, chunk: Rect, frame: &mut Frame) {
    let items = ["Tracks", "Albums", "Artists"].map(ListItem::new);

    let list = List::new(items)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">")
        .block(
            Block::default()
                .title("Library")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    frame.render_stateful_widget(list, chunk, &mut app.library_list_state);
}
