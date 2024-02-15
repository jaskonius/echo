use crate::config::Config;
use crate::{APP_NAME, CONFIG_FILE};
use anyhow::Result;
use ratatui::widgets::ListState;
use tracing::debug;

#[derive(PartialEq)]
pub enum HoveredSection {
    None,
    Library,
    Playlist,
    Main,
}

#[derive(PartialEq)]
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
            hovered_section: HoveredSection::Main,
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
        match self.hovered_section {
            HoveredSection::None => {}
            HoveredSection::Library => {}
            HoveredSection::Playlist => {}
            HoveredSection::Main => self.hovered_section = HoveredSection::Library,
        }

        match self.selected_section {
            SelectedSection::None => {}
            SelectedSection::Library => {
                self.selected_section = SelectedSection::None;
                self.hovered_section = HoveredSection::Library
            }
            SelectedSection::Playlist => {
                self.selected_section = SelectedSection::None;
                self.hovered_section = HoveredSection::Playlist
            }
            SelectedSection::Main => {
                self.selected_section = SelectedSection::None;
                self.hovered_section = HoveredSection::Main
            }
        }
    }

    pub fn down(&mut self) {
        match self.hovered_section {
            HoveredSection::None => {}
            HoveredSection::Library => self.hovered_section = HoveredSection::Playlist,
            HoveredSection::Playlist => {}
            HoveredSection::Main => {}
        }
    }

    pub fn up(&mut self) {
        match self.hovered_section {
            HoveredSection::None => {}
            HoveredSection::Library => {}
            HoveredSection::Playlist => self.hovered_section = HoveredSection::Library,
            HoveredSection::Main => {}
        }
    }

    pub fn right(&mut self) {
        match self.hovered_section {
            HoveredSection::None => {}
            HoveredSection::Library => self.hovered_section = HoveredSection::Main,
            HoveredSection::Playlist => self.hovered_section = HoveredSection::Main,
            HoveredSection::Main => {}
        }

        match self.selected_section {
            SelectedSection::None => {}
            SelectedSection::Library => {
                self.selected_section = SelectedSection::None;
                self.hovered_section = HoveredSection::Library
            }
            SelectedSection::Playlist => {
                self.selected_section = SelectedSection::None;
                self.hovered_section = HoveredSection::Playlist
            }
            SelectedSection::Main => {
                self.selected_section = SelectedSection::None;
                self.hovered_section = HoveredSection::Main
            }
        }
    }

    pub fn select(&mut self) {
        match self.hovered_section {
            HoveredSection::None => {}
            HoveredSection::Library => {
                self.hovered_section = HoveredSection::None;
                self.selected_section = SelectedSection::Library
            }
            HoveredSection::Playlist => {
                self.hovered_section = HoveredSection::None;
                self.selected_section = SelectedSection::Playlist
            }
            HoveredSection::Main => {
                self.hovered_section = HoveredSection::None;
                self.selected_section = SelectedSection::Main
            }
        }
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
