use crate::app::App;
use ratatui::layout::Rect;
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

pub fn render_searchbar(_app: &mut App, chunk: Rect, frame: &mut Frame) {
    let searchbar = Paragraph::new(Text::from("Search text")).block(
        Block::default()
            .title("Search")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(searchbar, chunk);
}
