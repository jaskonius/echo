use crate::app::App;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('a') => app.initial_state = false,
        _ => {}
    }

    Ok(())
}
