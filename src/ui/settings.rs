use crate::state::app_state::AppState;
use crate::state::helper;
use eframe::egui;
pub fn show(ctx: &egui::Context, state: &mut AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Settings");
        ui.add_space(8.0);

        ui.separator();
        ui.add_space(8.0);

        // Run in Background Toggle
        ui.checkbox(&mut state.run_in_background, "Run in background");
        ui.add_space(4.0);

        // Start at Startup Toggle
        ui.checkbox(&mut state.start_at_startup, "Launch on system startup");
        ui.add_space(4.0);

        ui.checkbox(&mut state.task_bar, "Taskbar icon tray");

        ui.add_space(20.0);
        if ui.button("Save Settings").clicked() {
            // Call the function from helper.rs
            helper::save_config(state);

            println!(
                "Success: Saved to config.toml (Background={}, Startup={})",
                state.run_in_background, state.start_at_startup
            );
        }
    });
}
