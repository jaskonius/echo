use crate::app::App;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Row, Table, TableState};
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

    fn truncate(&mut self, max_len: u16) {
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

pub fn render_center(_app: &mut App, chunk: Rect, frame: &mut Frame) {
    let mut rows = vec![
        RowData::new(
            String::from("Heart of Courage"),
            String::from("Two Steps from Hell"),
            String::from("Invincible"),
            String::from("3:12"),
        ),
        RowData::new(
            String::from("Time"),
            String::from("Hans Zimmer"),
            String::from("Inception (Original Motion Picture Soundtrack)"),
            String::from("4:35"),
        ),
        RowData::new(
            String::from("Test Drive"),
            String::from("John Powell"),
            String::from("How to Train Your Dragon (Music from the Motion Picture)"),
            String::from("3:15"),
        ),
        RowData::new(
            String::from("Victory"),
            String::from("Two Steps from Hell"),
            String::from("Archangel"),
            String::from("5:20"),
        ),
        RowData::new(
            String::from("Forbidden Friendship"),
            String::from("John Powell"),
            String::from("How to Train Your Dragon (Music from the Motion Picture)"),
            String::from("4:10"),
        ),
    ];
    rows.iter_mut()
        .for_each(|row| row.truncate(chunk.width / 4 - 3));

    let table = Table::new(rows.iter().map(|row| row.to_row()))
        .header(
            Row::new(["Title", "Artist", "Album", "Duration"])
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .widths(&[
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Length(8),
        ])
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">")
        .block(
            Block::default()
                .title("Play Queue")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    let mut table_state = TableState::default().with_selected(Some(3));

    frame.render_stateful_widget(table, chunk, &mut table_state);
}
