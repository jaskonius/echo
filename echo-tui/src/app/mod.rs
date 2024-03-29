use crate::app::stateful_list::StatefulList;
use crate::app::stateful_table::StatefulTable;
use crate::config::Config;
use crate::{APP_NAME, CONFIG_FILE};
use anyhow::Result;
use std::path::PathBuf;
use tracing::debug;

mod stateful_list;
pub mod stateful_table;

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

    /// Buffer for text input of `root_dir` path.
    pub root_dir_input_buffer: String,
    /// Whether the user is currently typing.
    pub root_dir_input_active: bool,
    /// Root directory of music.
    pub root_dir: PathBuf, // might move it into config for consistency

    /// Whether to show queue.
    pub show_queue: bool,
    pub active_main: ActiveMain,

    pub hovered_section: HoveredSection,
    pub selected_section: SelectedSection,

    pub library_list: StatefulList<'a>,
    pub playlist_list: StatefulList<'a>,

    pub center_table: StatefulTable,

    pub queue_table: StatefulTable,
}

impl<'a> App<'a> {
    pub fn from(config: Config) -> Result<Self> {
        config.validate()?;
        let config_clone = config.clone();
        Ok(Self {
            is_running: true,
            config,
            root_dir_input_buffer: String::new(),
            root_dir_input_active: false,
            root_dir: PathBuf::from(config_clone.root_dir),
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
            center_table: StatefulTable::new(vec![vec![]]),
            queue_table: StatefulTable::new(vec![
                vec![
                    "Last One Standing".to_string(),
                    "Two Steps from Hell".to_string(),
                    "Myth".to_string(),
                    "03:25".to_string(),
                ],
                vec![
                    "Insomnia".to_string(),
                    "2WEI".to_string(),
                    "Sequels".to_string(),
                    "03:26".to_string(),
                ],
                vec![
                    "Dark Matter".to_string(),
                    "Les Friction".to_string(),
                    "Dark Matter".to_string(),
                    "04:20".to_string(),
                ],
            ]),
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
            SelectedSection::Main => {
                if self.show_queue {
                    self.queue_table.next();
                } else {
                    self.center_table.next();
                }
            }
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
            SelectedSection::Main => {
                if self.show_queue {
                    self.queue_table.prev();
                } else {
                    self.center_table.prev();
                }
            }
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
                    self.center_table.state.select(Some(0)); // reset table state
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
            root_dir: self
                .root_dir
                .clone()
                .to_str()
                .expect("could not convert root_dir to str")
                .to_string(),
            key_bindings: self.config.key_bindings.clone(),
            hover_color: self.config.hover_color.clone(),
            progress_color: self.config.progress_color.clone(),
            selected_color: self.config.selected_color.clone(),
        };
        confy::store(APP_NAME, CONFIG_FILE, config).expect("Failed to store config");
    }
}
