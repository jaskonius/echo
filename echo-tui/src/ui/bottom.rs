use crate::app::App;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Gauge, Paragraph};
use ratatui::Frame;

pub fn render_bottom(app: &mut App, chunk: Rect, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Area for song and artist name
            Constraint::Percentage(25), // Padding
            Constraint::Percentage(25), // Area for progress bar
        ])
        .margin(1)
        .split(chunk);

    let title = "Playing (Shuffle: Off | Repeat: Off | Volume: 50%)";
    let bottom_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(bottom_block, chunk);
    render_song_and_artist(app, chunks[0], frame);
    render_progress_bar(app, chunks[2], frame);
}

fn render_song_and_artist(_app: &mut App, chunk: Rect, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Song
            Constraint::Percentage(50), // Artist
        ])
        .split(chunk);

    let song = "Victory";
    let artist = "Two Steps from Hell";

    let song = Paragraph::new(song).block(Block::default().borders(Borders::NONE));
    let artist = Paragraph::new(artist).block(Block::default().borders(Borders::NONE));

    frame.render_widget(song, chunks[0]);
    frame.render_widget(artist, chunks[1]);
}

fn render_progress_bar(app: &mut App, chunk: Rect, frame: &mut Frame) {
    let progress_percentage = ((3f32 * 60f32) / (5f32 * 60f32 + 20f32)) * 100f32;
    let progress_bar = Gauge::default()
        .percent(progress_percentage as u16)
        .label("3:00 / 5:20")
        .gauge_style(
            Style::default()
                .fg(app.config.progress_color.parse().expect("invalid color"))
                .bg(Color::Gray),
        );

    frame.render_widget(progress_bar, chunk);
}
