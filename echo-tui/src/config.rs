use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Config {
    pub show_greeting: bool,
    pub key_bindings: KeyBindings,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_greeting: true,
            key_bindings: Default::default(),
        }
    }
}

/// Key bindings used to navigate the app. All are configurable freely. See each field for their defaults.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct KeyBindings {
    /// Default: 'q'
    pub quit: char,

    /// Default: 'w'
    pub toggle_queue: char,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: 'q',
            toggle_queue: 'w',
        }
    }
}
