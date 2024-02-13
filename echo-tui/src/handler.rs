use crate::app::App;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use KeyCode::Char;

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
    if let Char(key) = key_event.code {
        if key == app.config.key_bindings.quit {
            app.quit();
        }
        if key == app.config.key_bindings.toggle_queue {
            app.toggle_queue();
        }
    }

    Ok(())
}
