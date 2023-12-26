use crate::app::App;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

pub fn render_bottom(_app: &mut App, chunk: Rect, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Area for song and artist name
            Constraint::Percentage(25), // Padding
            Constraint::Percentage(25), // Area for progress bar
        ])
        .margin(1)
        .split(chunk);
}

fn render_song_and_artist(app: &mut App, chunk: Rect, frame: &mut Frame) {}

fn render_progress_bar(app: &mut App, chunk: Rect, frame: &mut Frame) {}
