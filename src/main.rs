mod app;
mod services;
mod state;
mod ui;

use app::MyApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "My Native Rust App",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
