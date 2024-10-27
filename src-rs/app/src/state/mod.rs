mod app_state;

pub use app_state::*;

pub fn create_state() -> AppState {
    AppState::new()
}
