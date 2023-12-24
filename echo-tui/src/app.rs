pub struct App {
    is_running: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    // Getters and setters
    pub fn quit(&mut self) {
        self.is_running = false;
    }
    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

impl Default for App {
    fn default() -> Self {
        Self { is_running: true }
    }
}
