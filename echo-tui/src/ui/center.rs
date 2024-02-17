use crate::app::{ActiveMain, App, HoveredSection, SelectedSection};
use echo_localfiles::library;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::prelude::{Modifier, Style, Text};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, Wrap};
use ratatui::Frame;

struct RowData {
    title: String,
    artist: String,
    album: String,
    duration: String,
}

impl RowData {
    fn new(title: String, artist: String, album: String, duration: String) -> Self {
        Self {
            title,
            artist,
            album,
            duration,
        }
    }

    fn from(track: &[String; 4]) -> Self {
        Self::new(
            track[0].clone(),
            track[1].clone(),
            track[2].clone(),
            track[3].clone(),
        )
    }

    fn truncate(mut self, max_len: u16) -> Self {
        let max_len = max_len as usize;
        let ellipsis_bytes: Vec<u8> = vec![0xE2, 0x80, 0xA6]; // U+2026 -> horizontal ellipsis
        let ellipsis = std::str::from_utf8(&ellipsis_bytes).expect("could not decode ellipsis");

        if self.title.len() > max_len {
            self.title.truncate(max_len);
            self.title = self.title.trim_end().to_string();
            self.title.push_str(ellipsis);
        }

        if self.artist.len() > max_len {
            self.artist.truncate(max_len);
            self.artist = self.artist.trim_end().to_string();
            self.artist.push_str(ellipsis);
        }

        if self.album.len() > max_len {
            self.album.truncate(max_len);
            self.album = self.album.trim_end().to_string();
            self.album.push_str(ellipsis);
        }

        self
    }

    fn to_row(&self) -> Row {
        Row::new(vec![
            Cell::from(self.title.clone()),
            Cell::from(self.artist.clone()),
            Cell::from(self.album.clone()),
            Cell::from(self.duration.clone()),
        ])
    }
}

pub fn render_center(app: &mut App, chunk: Rect, frame: &mut Frame) {
    if app.show_queue {
        render_queue(app, chunk, frame)
    } else {
        render_active_main(app, chunk, frame)
    }
}

fn render_queue(app: &mut App, chunk: Rect, frame: &mut Frame) {
    let rows = app
        .queue_items
        .iter()
        .map(RowData::from)
        .map(|row| row.truncate(chunk.width / 4 - 3))
        .collect();

    render_table(
        "Queue",
        ["Title", "Artist", "Album", "Duration"].to_vec(),
        rows,
        app,
        chunk,
        frame,
    );
}

fn render_active_main(app: &mut App, chunk: Rect, frame: &mut Frame) {
    match app.active_main {
        ActiveMain::None => render_active_main_none(app, chunk, frame),
        ActiveMain::Library(idx) => render_active_main_library(idx, app, chunk, frame),
        ActiveMain::Playlists(idx) => render_active_main_playlists(idx, app, chunk, frame),
    }
}

fn render_active_main_none(app: &mut App, chunk: Rect, frame: &mut Frame) {
    let border_style = if app.hovered_section == HoveredSection::Main
        || app.selected_section == SelectedSection::Main
    {
        Style::default().fg(app.config.hover_color.parse().expect("invalid color"))
    } else {
        Style::default()
    };

    frame.render_widget(
        Paragraph::new(Text::raw(
            r"_______   ______  __    __    ______
|   ____| /      ||  |  |  |  /  __  \
|  |__   |  ,----'|  |__|  | |  |  |  |
|   __|  |  |     |   __   | |  |  |  |
|  |____ |  `----.|  |  |  | |  `--'  |
|_______| \______||__|  |__|  \______/

Nothing selected yet.

To navigate around, use j,k,l,h. To select something, press enter.",
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap::default()),
        chunk,
    );
}

fn render_active_main_library(idx: usize, app: &mut App, chunk: Rect, frame: &mut Frame) {
    match idx {
        0 => {
            let rows: Vec<_> = library::get_tracks()
                .iter()
                .map(RowData::from)
                .map(|row| row.truncate(chunk.width / 4 - 3))
                .collect();
            render_table(
                "Tracks",
                ["Title", "Artist", "Album", "Duration"].to_vec(),
                rows,
                app,
                chunk,
                frame,
            )
        }
        1 => {
            let rows: Vec<_> = library::get_albums()
                .iter()
                .map(RowData::from)
                .map(|row| row.truncate(chunk.width / 4 - 3))
                .collect();
            render_table(
                "Albums",
                ["Artist", "Album", "Year", "Duration"].to_vec(),
                rows,
                app,
                chunk,
                frame,
            )
        }
        2 => {
            let rows = vec![];
            render_table("Artists", ["Name"].to_vec(), rows, app, chunk, frame)
        }
        _ => unreachable!("3 or more only possible if library got an additional section"),
    }
}

fn render_active_main_playlists(_idx: usize, _app: &mut App, _chunk: Rect, _frame: &mut Frame) {
    todo!()
}

fn render_table(
    title: &str,
    header: Vec<&str>,
    rows: Vec<RowData>,
    app: &mut App,
    chunk: Rect,
    frame: &mut Frame,
) {
    let border_style = if app.hovered_section == HoveredSection::Main {
        Style::default().fg(app.config.hover_color.parse().expect("invalid color"))
    } else if app.selected_section == SelectedSection::Main {
        Style::default().fg(app.config.selected_color.parse().expect("invalid color"))
    } else {
        Style::default()
    };

    let table_highlight_style = if app.hovered_section == HoveredSection::Main {
        Style::default().add_modifier(Modifier::BOLD).fg(app
            .config
            .hover_color
            .parse()
            .expect("invalid color"))
    } else {
        Style::default().add_modifier(Modifier::BOLD)
    };

    let table = Table::new(rows.iter().map(|row| row.to_row()))
        .header(
            Row::new(header)
                .style(Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
        )
        .widths(&[
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Length(8),
        ])
        .highlight_style(table_highlight_style)
        .highlight_symbol(">")
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        );

    frame.render_stateful_widget(table, chunk, &mut app.table_state);
}
