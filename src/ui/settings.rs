use crate::state::app_state::AppState;
use crate::state::helper::{self, apply_startup};
use eframe::egui::{self, Color32, RichText};

pub fn show(ctx: &egui::Context, state: &mut AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // Outer padding frame for a cleaner layout
        egui::Frame::NONE
            .inner_margin(egui::Margin {
                left: 16,
                right: 16,
                top: 16,
                bottom: 16,
            })
            .show(ui, |ui| {
                // --- Header Area ---
                ui.heading(RichText::new("⚙ Settings").size(28.0).strong());
                ui.add_space(8.0);
                ui.label(
                    RichText::new("Customize the application's behavior and startup options.")
                        .color(Color32::GRAY),
                );

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                // --- Application Behavior ---
                ui.label(RichText::new("Application Behavior").size(18.0).strong());
                ui.add_space(12.0);

                // Make the group take full available width
                egui::Frame::group(ui.style())
                    .fill(ui.visuals().extreme_bg_color)
                    .corner_radius(8.0)
                    .inner_margin(egui::Margin::same(16))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.spacing_mut().item_spacing.y = 12.0;

                        // Run in background
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut state.run_in_background, "");
                            ui.vertical(|ui| {
                                ui.label(RichText::new("Run in background").size(15.0));
                                ui.label(
                                    RichText::new(
                                        "Keep the application running when the window is closed.",
                                    )
                                    .small()
                                    .color(Color32::GRAY),
                                );
                            });
                        });

                        ui.separator();

                        // Launch on system startup
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut state.start_at_startup, "");
                            ui.vertical(|ui| {
                                ui.label(RichText::new("Launch on system startup").size(15.0));
                                ui.label(
                                    RichText::new(
                                        "Automatically start the application when you log in.",
                                    )
                                    .small()
                                    .color(Color32::GRAY),
                                );
                            });
                        });

                        ui.separator();

                        // Taskbar tray icon
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut state.task_bar, "");
                            ui.vertical(|ui| {
                                ui.label(RichText::new("Enable taskbar tray icon").size(15.0));
                                ui.label(
                                    RichText::new(
                                        "Show an icon in the system tray for quick access.",
                                    )
                                    .small()
                                    .color(Color32::GRAY),
                                );
                            });
                        });
                    });

                ui.add_space(32.0);

                // --- Save Button ---
                ui.vertical_centered(|ui| {
                    let save_btn =
                        egui::Button::new(RichText::new("💾 Save Settings").size(16.0).strong())
                            .min_size(egui::vec2(200.0, 40.0))
                            .corner_radius(8.0);

                    if ui
                        .add(save_btn)
                        .on_hover_text("Apply and save all settings")
                        .clicked()
                    {
                        helper::save_config(state);
                        apply_startup(state.start_at_startup);

                        println!(
                            "Saved (Background={}, Startup={}, Taskbar={})",
                            state.run_in_background, state.start_at_startup, state.task_bar
                        );
                    }
                });
            });
    });
}
