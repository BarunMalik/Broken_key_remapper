use crate::state::app_state::AppState;
use eframe::egui;

pub fn show(ctx: &egui::Context, state: &mut AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Home Screen");

        ui.horizontal(|ui| {
            ui.label("Enter name:");
            ui.text_edit_singleline(&mut state.username);
        });

        if ui.button("Greet").clicked() {
            println!("Hello, {}", state.username);
        }
    });
}
