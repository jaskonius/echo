use crate::config::Config;
use crate::{APP_NAME, CONFIG_FILE};

pub struct App {
    pub is_running: bool,

    /// see [`Config::show_greeting`]
    pub show_greeting: bool,
}

impl App {
    pub fn from(config: Config) -> Self {
        Self {
            is_running: true,
            show_greeting: config.show_greeting,
        }
    }

    pub fn quit(&mut self) {
        self.is_running = false;

        let config = Config {
            show_greeting: self.show_greeting,
        };
        confy::store(APP_NAME, CONFIG_FILE, config).expect("Failed to store config");
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            is_running: true,
            show_greeting: true,
        }
    }
}
