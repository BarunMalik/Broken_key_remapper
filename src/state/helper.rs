use crate::state::app_state::AppState;
use std::fs; // Pointing to your AppState struct

pub fn save_config(state: &AppState) {
    let path = "config.toml";
    if let Ok(toml_string) = toml::to_string_pretty(state) {
        let _ = fs::write(path, toml_string);
    }
}

pub fn load_config() -> AppState {
    let path = "config.toml";
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(config) = toml::from_str(&content) {
            return config;
        }
    }
    AppState::default()
}
