use crate::config::Config;
use crate::{APP_NAME, CONFIG_FILE};
use tracing::debug;

/// Keeps track of application state.
pub struct App {
    /// Whether app is running. App will quit when set to false. Should **not** be set manually,
    /// use [`app.quit()`](Self::quit) instead.
    pub is_running: bool,

    /// Configuration loaded from config file.
    pub config: Config,

    /// Queue can be toggled, see [`Config::key_bindings`]
    pub show_queue: bool,
}

impl App {
    pub fn from(config: Config) -> Self {
        Self {
            is_running: true,
            config,
            show_queue: false,
        }
    }

    pub fn toggle_queue(&mut self) {
        debug!(
            "toggle queue from {} to {}",
            self.show_queue, !self.show_queue
        );
        self.show_queue = !self.show_queue
    }

    pub fn quit(&mut self) {
        self.is_running = false;

        let config = Config {
            show_greeting: false,
            key_bindings: self.config.key_bindings,
        };
        confy::store(APP_NAME, CONFIG_FILE, config).expect("Failed to store config");
    }
}
