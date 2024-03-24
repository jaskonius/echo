use crate::app::{App, HoveredSection, SelectedSection};
use echo_localfiles::playlists::get_playlists;
use ratatui::layout::Rect;
use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, List};
use ratatui::Frame;

pub fn render_playlists(app: &mut App, chunk: Rect, frame: &mut Frame) {
    let border_style = if app.hovered_section == HoveredSection::Playlist {
        Style::default().fg(app.config.hover_color.parse().expect("invalid color"))
    } else if app.selected_section == SelectedSection::Playlist {
        Style::default().fg(app.config.selected_color.parse().expect("invalid color"))
    } else {
        Style::default()
    };

    let list_highlight_style = if app.hovered_section == HoveredSection::Playlist {
        Style::default().add_modifier(Modifier::BOLD).fg(app
            .config
            .hover_color
            .parse()
            .expect("invalid color"))
    } else {
        Style::default().add_modifier(Modifier::BOLD)
    };

    app.playlist_list.update(get_playlists());
    let list = List::new(app.playlist_list.items.clone())
        .highlight_style(list_highlight_style)
        .highlight_symbol(">")
        .block(
            Block::default()
                .title("Playlists")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        );

    frame.render_stateful_widget(list, chunk, &mut app.playlist_list.state);
}
