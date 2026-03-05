use crate::state::app_state::AppState;
use std::fs;

#[cfg(target_os = "windows")]
use auto_launcher::AutoLaunchBuilder;

const APP_NAME: &str = "BekarKeyboard";

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

/// Returns true if the app was launched via the Windows startup registry entry
/// (i.e. the --minimized flag was passed as a CLI argument).
pub fn was_auto_launched() -> bool {
    #[cfg(target_os = "windows")]
    {
        return std::env::args().any(|a| a == "--minimized");
    }
    #[cfg(not(target_os = "windows"))]
    {
        return false;
    }
}

#[cfg(target_os = "windows")]
pub fn apply_startup(enabled: bool) {
    let app_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("[Startup] Failed to get executable path: {}", e);
            return;
        }
    };

    let app_path_str = match app_path.to_str() {
        Some(s) => s,
        None => {
            eprintln!("[Startup] Executable path contains invalid Unicode.");
            return;
        }
    };

    let auto = match AutoLaunchBuilder::new()
        .set_app_name(APP_NAME)
        .set_app_path(app_path_str)
        .set_args(&["--minimized"])
        .build()
    {
        Ok(a) => a,
        Err(e) => {
            eprintln!("[Startup] Failed to build AutoLaunch: {}", e);
            return;
        }
    };

    if enabled {
        match auto.enable() {
            Ok(_) => println!("[Startup] Startup entry enabled for '{}'.", APP_NAME),
            Err(e) => eprintln!("[Startup] Failed to enable startup: {}", e),
        }
    } else {
        match auto.disable() {
            Ok(_) => println!("[Startup] Startup entry disabled for '{}'.", APP_NAME),
            Err(e) => eprintln!("[Startup] Failed to disable startup: {}", e),
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn apply_startup(_enabled: bool) {
    eprintln!("[Startup] Auto-launch is only supported on Windows.");
}
