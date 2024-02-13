use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub show_greeting: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_greeting: true,
        }
    }
}
