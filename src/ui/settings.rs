use crate::state::app_state::AppState;
use crate::state::helper;
use eframe::egui;

pub fn show(ctx: &egui::Context, state: &mut AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("⚙ Settings");
        });

        ui.add_space(15.0);

        egui::Frame::group(ui.style())
            .fill(ui.visuals().extreme_bg_color)
            .corner_radius(10.0)
            .inner_margin(egui::Margin::same(18))
            .show(ui, |ui| {

                ui.label(
                    egui::RichText::new("Application Behavior")
                        .strong()
                        .size(16.0),
                );

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                ui.checkbox(&mut state.run_in_background, "Run in background");
                ui.checkbox(&mut state.start_at_startup, "Launch on system startup");
                ui.checkbox(&mut state.task_bar, "Enable taskbar tray icon");
            });

        ui.add_space(25.0);

        ui.vertical_centered(|ui| {
            if ui
                .add_sized(
                    [200.0, 40.0],
                    egui::Button::new(
                        egui::RichText::new("💾 Save Settings").size(16.0),
                    ),
                )
                .clicked()
            {
                helper::save_config(state);

                println!(
                    "Saved (Background={}, Startup={})",
                    state.run_in_background,
                    state.start_at_startup
                );
            }
        });
    });
}