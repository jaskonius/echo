use crate::app::{App, HoveredSection, SelectedSection};
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, List};
use ratatui::Frame;

pub fn render_library(app: &mut App, chunk: Rect, frame: &mut Frame) {
    let border_style = if app.hovered_section == HoveredSection::Library {
        Style::default().fg(app.config.hover_color.parse().expect("invalid color"))
    } else if app.selected_section == SelectedSection::Library {
        Style::default().fg(app.config.selected_color.parse().expect("invalid color"))
    } else {
        Style::default()
    };

    let list_highlight_style = if app.hovered_section == HoveredSection::Library {
        Style::default().add_modifier(Modifier::BOLD).fg(app
            .config
            .hover_color
            .parse()
            .expect("invalid color"))
    } else {
        Style::default().add_modifier(Modifier::BOLD)
    };

    let list = List::new(app.library_list.items.clone())
        .highlight_style(list_highlight_style)
        .highlight_symbol(">")
        .block(
            Block::default()
                .title("Library")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        );

    frame.render_stateful_widget(list, chunk, &mut app.library_list.state);
}
