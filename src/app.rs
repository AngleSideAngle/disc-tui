struct AppState;

impl Default for AppState {
    fn default() -> Self {
        Self {  }
    }
}

pub struct App {
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        let state = AppState::default();
        Self { state }
    }

    pub fn state(&self) -> &AppState {
        &self.state()
    }
}