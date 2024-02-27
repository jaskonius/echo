mod bottom;
mod center;
mod library;
mod playlists;
mod searchbar;

use crate::app::App;
use crate::ui::bottom::render_bottom;
use crate::ui::center::render_center;
use crate::ui::library::render_library;
use crate::ui::playlists::render_playlists;
use crate::ui::searchbar::render_searchbar;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::Frame;
pub fn render(app: &mut App, frame: &mut Frame) {
    if app.config.show_greeting {
        render_greeting(frame);
        return;
    }
    if !app.root_dir.exists() {
        render_root_dir_input(app, frame);
        return;
    }
    render_app(app, frame);
}

fn render_app(app: &mut App, frame: &mut Frame) {
    let app_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search bar
            Constraint::Min(0),    // Main section
            Constraint::Length(6), // Bottom bar
        ])
        .split(frame.size());

    render_searchbar(app, app_layout[0], frame);
    render_main(app, app_layout[1], frame);
    render_bottom(app, app_layout[2], frame);
}

fn render_main(app: &mut App, rect: Rect, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // library and playlists
            Constraint::Percentage(75), // center
        ])
        .split(rect);

    let left_side = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // library
            Constraint::Min(0),    // playlists
        ])
        .split(layout[0]);

    render_library(app, left_side[0], frame);
    render_playlists(app, left_side[1], frame);
    render_center(app, layout[1], frame);
}
fn render_greeting(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(Text::raw(
            r"_______   ______  __    __    ______
|   ____| /      ||  |  |  |  /  __  \
|  |__   |  ,----'|  |__|  | |  |  |  |
|   __|  |  |     |   __   | |  |  |  |
|  |____ |  `----.|  |  |  | |  `--'  |
|_______| \______||__|  |__|  \______/

 Welcome to Echo, your neat little command line music player!

 This project is in very early development, expect bugs and shitty performance. I use this project to learn Rust and stuff along the way.
 If you have suggestions, want to provide feedback or run into a bug (which is very likely), please open an issue on GitHub at https://github.com/jaskonius/echo.

 Press any key to continue.")).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        ).alignment(Alignment::Center).wrap(Wrap::default()), frame.size(),
    );
}

fn render_root_dir_input(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0)
        ])
        .margin(2)
        .split(frame.size());

    app.root_dir_input_active = true;
    // `Line` with vec of `Span` might be better
    let searchbar = Paragraph::new(Text::from(format!(
        "{}_",
        app.root_dir_input_buffer.clone()
    )))
    .block(
        Block::default()
            .title("Enter absolut path to root dir.")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(searchbar, chunks[0]);
}
