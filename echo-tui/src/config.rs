use anyhow::{anyhow, Context, Result};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Show greeting on start. Set to false by [`handler`](crate::handler::handle_key_events) after an event has occured.
    pub show_greeting: bool,
    pub hover_color: String,
    pub selected_color: String,
    pub progress_color: String,
    pub key_bindings: KeyBindings,
}

impl Config {
    /// Checks config values for validity.
    ///
    /// # Errors
    /// Returns the error first occurred.
    pub fn validate(&self) -> Result<()> {
        Color::from_str(&self.hover_color).context("hover_color invalid")?;
        Color::from_str(&self.selected_color).context("selected_color invalid")?;
        Color::from_str(&self.progress_color).context("progress_color invalid")?;

        let mut set_of_keys = HashSet::new();
        if !set_of_keys.insert(self.key_bindings.quit) {
            return Err(anyhow!("key for quit already assigned"));
        };
        if !set_of_keys.insert(self.key_bindings.toggle_queue) {
            return Err(anyhow!("key for toggle_queue already used"));
        };
        if !set_of_keys.insert(self.key_bindings.left) {
            return Err(anyhow!("key for left already used"));
        };
        if !set_of_keys.insert(self.key_bindings.down) {
            return Err(anyhow!("key for down already used"));
        };
        if !set_of_keys.insert(self.key_bindings.up) {
            return Err(anyhow!("key for up already used"));
        };
        if !set_of_keys.insert(self.key_bindings.right) {
            return Err(anyhow!("key for right already used"));
        };

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_greeting: true,
            hover_color: "blue".to_string(),
            selected_color: "yellow".to_string(),
            progress_color: "blue".to_string(),
            key_bindings: Default::default(),
        }
    }
}

/// Key bindings used to navigate the app. All are configurable freely. See each field for their defaults.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyBindings {
    /// Default: 'q'
    pub quit: char,

    /// Default: 'w'
    pub toggle_queue: char,

    /// Default: 'h'
    pub left: char,

    /// Default: 'j'
    pub down: char,

    /// Default: 'k'
    pub up: char,

    /// Default: 'l'
    pub right: char,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: 'q',
            toggle_queue: 'w',
            left: 'h',
            down: 'j',
            up: 'k',
            right: 'l',
        }
    }
}
