use crate::app::stateful_list::StatefulList;
use crate::config::Config;
use crate::{APP_NAME, CONFIG_FILE};
use anyhow::Result;
use ratatui::widgets::TableState;
use tracing::debug;

mod stateful_list;
mod stateful_table;

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

/// What to show in main section.
///
/// Queue is treated as an overlay, so it can get toggled via `show_queue`.
#[derive(PartialEq)]
pub enum ActiveMain {
    None,
    Library(usize),
    Playlists(usize),
}

/// Keeps track of application state.
pub struct App<'a> {
    /// Whether app is running. App will quit when set to false. Should **not** be set manually,
    /// use [`app.quit()`](Self::quit) instead.
    pub is_running: bool,

    /// Configuration loaded from config file.
    pub config: Config,

    /// Whether to show queue.
    pub show_queue: bool,
    pub active_main: ActiveMain,

    pub hovered_section: HoveredSection,
    pub selected_section: SelectedSection,

    pub library_list: StatefulList<'a>,
    pub playlist_list: StatefulList<'a>,

    pub table_items: Vec<[String; 4]>,
    pub table_state: TableState,

    pub queue_items: Vec<[String; 4]>,
}

impl<'a> App<'a> {
    pub fn from(config: Config) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            is_running: true,
            config,
            show_queue: false,
            active_main: ActiveMain::None,
            hovered_section: HoveredSection::Main,
            selected_section: SelectedSection::None,
            library_list: StatefulList::new(vec![
                "Tracks".to_string(),
                "Albums".to_string(),
                "Artists".to_string(),
            ]),
            playlist_list: StatefulList::new(vec![]),
            table_items: vec![],
            table_state: TableState::default().with_selected(Some(0)),
            queue_items: vec![[
                "Last One Standing".to_string(),
                "Two Steps from Hell".to_string(),
                "Myth".to_string(),
                "03:25".to_string(),
            ]],
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

        match self.selected_section {
            SelectedSection::None => {}
            SelectedSection::Library => {
                self.library_list.next();
            }
            SelectedSection::Playlist => {
                self.playlist_list.next();
            }
            SelectedSection::Main => {}
        }
    }

    pub fn up(&mut self) {
        match self.hovered_section {
            HoveredSection::None => {}
            HoveredSection::Library => {}
            HoveredSection::Playlist => self.hovered_section = HoveredSection::Library,
            HoveredSection::Main => {}
        }

        match self.selected_section {
            SelectedSection::None => {}
            SelectedSection::Library => {
                self.library_list.prev();
            }
            SelectedSection::Playlist => {
                self.playlist_list.prev();
            }
            SelectedSection::Main => {}
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
        // needs to be dealt with first so that both matches don't influence each other
        match self.selected_section {
            SelectedSection::None => {}
            SelectedSection::Library => {
                if let Some(selected_list_item) = self.library_list.state.selected() {
                    self.active_main = ActiveMain::Library(selected_list_item);
                    self.selected_section = SelectedSection::Main;
                };
            }
            SelectedSection::Playlist => {}
            SelectedSection::Main => {}
        }

        match self.hovered_section {
            HoveredSection::None => {}
            HoveredSection::Library => {
                self.hovered_section = HoveredSection::None;
                self.selected_section = SelectedSection::Library;
            }
            HoveredSection::Playlist => {
                self.hovered_section = HoveredSection::None;
                self.selected_section = SelectedSection::Playlist;
            }
            HoveredSection::Main => {
                // shows info screen which cannot be selected
                if !self.show_queue && self.active_main == ActiveMain::None {
                    return;
                }
                self.hovered_section = HoveredSection::None;
                self.selected_section = SelectedSection::Main;
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
