use crate::app::App;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use std::path::PathBuf;
use tracing::info;
use KeyCode::Char;

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
    if app.root_dir_input_active {
        match key_event.code {
            Char(key) => {
                app.root_dir_input_buffer.push(key);
            }
            KeyCode::Backspace => {
                app.root_dir_input_buffer.pop();
            }
            KeyCode::Enter => {
                let path = PathBuf::from(app.root_dir_input_buffer.clone());
                if !path.exists() {
                    todo!("handle wrong input");
                }
                app.root_dir_input_active = false;
                app.root_dir.clone_from(&path);
                info!("set {:?} as root_dir", path);
            }
            KeyCode::Esc => app.quit(),
            _ => {}
        }

        return Ok(());
    }

    match key_event.code {
        KeyCode::Backspace => {}
        KeyCode::Enter => app.select(),
        KeyCode::Left => {}
        KeyCode::Right => {}
        KeyCode::Up => {}
        KeyCode::Down => {}
        KeyCode::Home => {}
        KeyCode::End => {}
        KeyCode::PageUp => {}
        KeyCode::PageDown => {}
        KeyCode::Tab => {}
        KeyCode::BackTab => {}
        KeyCode::Delete => {}
        KeyCode::Insert => {}
        KeyCode::F(_) => {}
        Char(key) => {
            if key == app.config.key_bindings.quit {
                app.quit();
            }

            if key == app.config.key_bindings.toggle_queue {
                app.toggle_queue();
            }

            if key == app.config.key_bindings.left {
                app.left();
            }

            if key == app.config.key_bindings.down {
                app.down();
            }

            if key == app.config.key_bindings.up {
                app.up();
            }

            if key == app.config.key_bindings.right {
                app.right();
            }
        }
        KeyCode::Null => {}
        KeyCode::Esc => app.quit(),
        KeyCode::CapsLock => {}
        KeyCode::ScrollLock => {}
        KeyCode::NumLock => {}
        KeyCode::PrintScreen => {}
        KeyCode::Pause => {}
        KeyCode::Menu => {}
        KeyCode::KeypadBegin => {}
        KeyCode::Media(_) => {}
        KeyCode::Modifier(_) => {}
    }

    app.config.show_greeting = false;

    Ok(())
}
