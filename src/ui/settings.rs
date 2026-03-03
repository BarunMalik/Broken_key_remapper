use crate::state::app_state::AppState;
use eframe::egui;

pub fn show(ctx: &egui::Context, state: &mut AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Settings");

        ui.label(format!("Current user: {}", state.username));
    });
}
