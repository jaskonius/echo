use anyhow::{Context, Result};

use log::info;
use ratatui::backend::CrosstermBackend;
use std::fs::File;
use std::io;
use std::sync::Arc;
use tracing_subscriber::{
    filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt, Layer,
};

use echo_tui::app::App;
use echo_tui::event::{Event, EventHandler};
use echo_tui::handler::handle_key_events;
use echo_tui::tui::Tui;

fn main() -> Result<()> {
    init_logging(File::create("echo-tui.log").context("Failed to create log file")?);
    info!("starting echo-tui");

    let mut app = App::new();

    // setup stuff
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = ratatui::Terminal::new(backend).context("Failed to create terminal")?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init().context("Failed to initialize terminal")?;

    // main loop
    while app.is_running() {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key) => {
                handle_key_events(key, &mut app).expect("Failed to handle key event");
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit().context("Failed to exit terminal")
}

fn init_logging(file: File) {
    let file_log = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(Arc::new(file));
    tracing_subscriber::registry()
        .with(file_log.with_filter(LevelFilter::DEBUG))
        .init();
}
