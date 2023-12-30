use crate::app::App;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, ListState};
use ratatui::Frame;

pub fn render_library(_app: &mut App, chunk: Rect, frame: &mut Frame) {
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

    let mut state = ListState::default().with_selected(Some(1));

    frame.render_stateful_widget(list, chunk, &mut state);
}
