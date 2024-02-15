use crate::app::{App, HoveredSection};
use ratatui::layout::Rect;
use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};
use ratatui::Frame;

pub fn render_playlists(app: &mut App, chunk: Rect, frame: &mut Frame) {
    let items = ["Playlist 1", "Playlist 2", "Playlist 3"].map(ListItem::new);

    let border_style = if app.hovered_section == HoveredSection::Playlist {
        Style::default().fg(app.config.hover_color.parse().expect("invalid color"))
    } else {
        Style::default()
    };

    let list_highlight_style = if app.hovered_section == HoveredSection::Playlist {
        Style::default()
            .fg(app.config.hover_color.parse().expect("invalid color"))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().add_modifier(Modifier::BOLD)
    };

    let list = List::new(items)
        .highlight_style(list_highlight_style)
        .highlight_symbol(">")
        .block(
            Block::default()
                .title("Playlists")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        );

    frame.render_stateful_widget(list, chunk, &mut app.playlist_list_state);
}
