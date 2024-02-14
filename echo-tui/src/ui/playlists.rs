use crate::app::App;
use ratatui::layout::Rect;
use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};
use ratatui::Frame;

pub fn render_playlists(app: &mut App, chunk: Rect, frame: &mut Frame) {
    let items = ["Playlist 1", "Playlist 2", "Playlist 3"].map(ListItem::new);

    let list = List::new(items)
        .highlight_style(
            Style::default().add_modifier(Modifier::BOLD).fg(app
                .config
                .hover_color
                .parse()
                .expect("invalid color")),
        )
        .highlight_symbol(">")
        .block(
            Block::default()
                .title("Playlists")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    frame.render_stateful_widget(list, chunk, &mut app.playlist_list_state);
}
