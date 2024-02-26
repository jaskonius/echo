use crate::app::stateful_table::RowData;
use crate::app::{ActiveMain, App, HoveredSection, SelectedSection};
use echo_localfiles::library;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::prelude::{Modifier, Style, Text};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Row, Table, Wrap};
use ratatui::Frame;

pub fn render_center(app: &mut App, chunk: Rect, frame: &mut Frame) {
    if app.show_queue {
        render_queue(app, chunk, frame)
    } else {
        render_active_main(app, chunk, frame)
    }
}

fn render_queue(app: &mut App, chunk: Rect, frame: &mut Frame) {
    let rows = app.queue_table.clone().truncate(chunk.width / 4 - 3).items;
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
            let tracks = library::tracks::get_tracks();
            app.center_table.items = tracks.iter().map(|t| RowData::new(t.clone())).collect();

            let rows = app.center_table.clone().truncate(chunk.width / 4 - 3).items;
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
            let tracks = library::albums::get_albums();
            app.center_table.items = tracks.iter().map(|t| RowData::new(t.clone())).collect();

            let rows = app.center_table.clone().truncate(chunk.width / 4 - 3).items;
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

    let table_state = if app.show_queue {
        &app.queue_table.state
    } else {
        &app.center_table.state
    };
    frame.render_stateful_widget(table, chunk, &mut table_state.clone());
}
