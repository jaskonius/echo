use anyhow::{Context, Result};

use ratatui::backend::CrosstermBackend;

use std::{fs, io};
use tracing::info;

use tracing_subscriber::filter::LevelFilter;

use echo_tui::app::App;
use echo_tui::config::Config;
use echo_tui::event::{Event, EventHandler};
use echo_tui::handler::handle_key_events;
use echo_tui::tui::Tui;
use echo_tui::{APP_NAME, CONFIG_FILE};

fn main() -> Result<()> {
    setup_logging();

    info!("starting echo-tui");

    let config: Config = confy::load(APP_NAME, CONFIG_FILE).context("Failed to load config")?;
    info!("using config: {:?}", config);

    let mut app = App::from(config).context("Failed to initialize app")?;

    // setup stuff
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = ratatui::Terminal::new(backend).context("Failed to create terminal")?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init().context("Failed to initialize terminal")?;

    // main loop
    while app.is_running {
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

fn setup_logging() {
    let config_dir = confy::get_configuration_file_path(APP_NAME, CONFIG_FILE)
        .expect("Failed to get path to config file")
        .parent()
        .expect("Failed to get parent directory")
        .to_path_buf();

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }

    // reminder: truncates file if already exists
    let log_file =
        fs::File::create(config_dir.join("echo.log")).expect("Failed to create log file");

    // TODO: is there a better way to check for debug build vs release build?
    #[cfg(debug_assertions)]
    let log_level = LevelFilter::DEBUG;
    #[cfg(not(debug_assertions))]
    let log_level = LevelFilter::INFO;

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_writer(log_file)
        .init();
}
