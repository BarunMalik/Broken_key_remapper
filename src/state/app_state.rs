use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)] // Added Clone for easier handling
pub struct AppState {
    pub current_screen: String,
    pub username: String,
    pub run_in_background: bool,
    pub start_at_startup: bool,
    pub task_bar: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_screen: "Home".to_string(),
            username: "Guest".to_string(),
            run_in_background: false,
            start_at_startup: false,
            task_bar: true,
        }
    }
}
