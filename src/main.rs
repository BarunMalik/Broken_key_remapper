mod app;
mod services;
mod state;
mod ui;

use app::MyApp;
fn main() -> Result<(), eframe::Error> {
    let initial_state = state::helper::load_config();
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "My Native Rust App",
        options,
        Box::new(|cc| {
            // Pass the initial_state INTO the new function here
            Ok(Box::new(MyApp::new(cc, initial_state)))
        }),
    )
}
