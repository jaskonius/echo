pub struct App {
    pub is_running: bool,

    /// whether or not app is in initial state
    pub initial_state: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            is_running: true,
            initial_state: true,
        }
    }
}
