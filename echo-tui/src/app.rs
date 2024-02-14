use crate::config::Config;
use crate::{APP_NAME, CONFIG_FILE};
use anyhow::Result;
use ratatui::widgets::ListState;
use tracing::debug;

pub enum HoveredSection {
    None,
    Library,
    Playlist,
    Main,
}

pub enum SelectedSection {
    None,
    Library,
    Playlist,
    Main,
}

pub enum HoveredItem {
    None,
    LibraryItem(usize),
    PlaylistItem(usize),
    MainItem(usize),
}

/// Keeps track of application state.
pub struct App {
    /// Whether app is running. App will quit when set to false. Should **not** be set manually,
    /// use [`app.quit()`](Self::quit) instead.
    pub is_running: bool,

    /// Configuration loaded from config file.
    pub config: Config,

    /// Queue can be toggled, see [`Config::key_bindings`]
    pub show_queue: bool,

    pub hovered_section: HoveredSection,
    pub selected_section: SelectedSection,
    pub hovered_item: HoveredItem,

    pub library_list_state: ListState,
    pub playlist_list_state: ListState,
}

impl App {
    pub fn from(config: Config) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            is_running: true,
            config,
            show_queue: false,
            hovered_section: HoveredSection::None,
            selected_section: SelectedSection::None,
            hovered_item: HoveredItem::None,
            library_list_state: ListState::default().with_selected(Some(0)),
            playlist_list_state: ListState::default().with_selected(Some(0)),
        })
    }

    pub fn toggle_queue(&mut self) {
        debug!(
            "toggle queue from {} to {}",
            self.show_queue, !self.show_queue
        );
        self.show_queue = !self.show_queue
    }

    pub fn left(&mut self) {
        todo!()
    }

    pub fn down(&mut self) {
        todo!()
    }

    pub fn up(&mut self) {
        todo!()
    }

    pub fn right(&mut self) {
        todo!()
    }

    pub fn quit(&mut self) {
        self.is_running = false;

        let config = Config {
            show_greeting: false,
            key_bindings: self.config.key_bindings.clone(),
            hover_color: self.config.hover_color.clone(),
            selected_color: self.config.selected_color.clone(),
        };
        confy::store(APP_NAME, CONFIG_FILE, config).expect("Failed to store config");
    }
}
