use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct KeyMap {
    pub broken_key: String,
    pub replacement_key: String,
    #[serde(default = "default_true")]
    pub tap_once: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppState {
    // UI-only states (Not saved to disk)
    #[serde(skip)]
    pub current_screen: String,

    #[serde(skip)]
    pub listener_enabled: bool,

    #[serde(skip)]
    pub listener_restore_after_recording: bool,

    #[serde(skip)]
    pub mapping_record_target: Option<(usize, bool)>,
    // Persistent settings (Saved to disk)
    pub run_in_background: bool,
    pub start_at_startup: bool,
    pub task_bar: bool,

    pub mappings: Vec<KeyMap>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_screen: "Home".to_string(),
            run_in_background: false,
            listener_enabled: false,
            listener_restore_after_recording: false,
            mapping_record_target: None,
            start_at_startup: false,
            task_bar: true,
            mappings: Vec::new(),
        }
    }
}
