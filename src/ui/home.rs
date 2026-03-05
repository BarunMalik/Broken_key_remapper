use crate::state::app_state::{AppState, KeyMap};
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
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("⌨ Key Remapper").size(28.0).strong());

                    // Listener Toggle Button (Right Aligned)
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (text, color) = if state.listener_enabled {
                            ("🟢 Active", Color32::from_rgb(100, 255, 100))
                        } else {
                            ("🔴 Paused", Color32::from_rgb(255, 100, 100))
                        };

                        let toggle_btn =
                            egui::Button::new(RichText::new(text).color(color).size(16.0))
                                .min_size(egui::vec2(100.0, 32.0))
                                .corner_radius(6.0);

                        if ui
                            .add(toggle_btn)
                            .on_hover_text("Toggle keyboard listener")
                            .clicked()
                        {
                            state.listener_enabled = !state.listener_enabled;
                        }
                    });
                });

                ui.add_space(8.0);
                ui.label(
                    RichText::new("Remap broken keys or create custom shortcuts.")
                        .color(Color32::GRAY),
                );

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                // --- Mappings Header ---
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Your Mappings").size(18.0).strong());

                    // Add Button (Right Aligned)
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let add_btn = egui::Button::new(RichText::new("➕ Add Mapping").strong())
                            .min_size(egui::vec2(120.0, 28.0))
                            .corner_radius(6.0);

                        if ui.add(add_btn).clicked() {
                            state.mappings.push(KeyMap {
                                broken_key: "".to_string(),
                                replacement_key: "".to_string(),
                            });
                        }
                    });
                });

                ui.add_space(16.0);

                // --- Mappings List ---
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        if state.mappings.is_empty() {
                            // Empty State
                            ui.add_space(60.0);
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    RichText::new("No mappings yet.")
                                        .size(16.0)
                                        .color(Color32::GRAY),
                                );
                                ui.add_space(8.0);
                                ui.label(
                                    RichText::new("Click 'Add Mapping' to get started!")
                                        .color(Color32::DARK_GRAY),
                                );
                            });
                        } else {
                            let mut to_remove = None;

                            for (idx, map) in state.mappings.iter_mut().enumerate() {
                                ui.group(|ui| {
                                    // Make the group take full available width
                                    ui.set_width(ui.available_width());

                                    ui.horizontal(|ui| {
                                        // Left side: Broken Key
                                        ui.vertical(|ui| {
                                            ui.label(
                                                RichText::new("Broken Key")
                                                    .small()
                                                    .color(Color32::GRAY),
                                            );
                                            ui.add(
                                                egui::TextEdit::singleline(&mut map.broken_key)
                                                    .desired_width(140.0)
                                                    .hint_text("e.g. A"),
                                            );
                                        });

                                        ui.add_space(12.0);

                                        // Arrow Indicator
                                        ui.vertical(|ui| {
                                            ui.add_space(16.0); // Align with text box visually
                                            ui.label(RichText::new("➡").size(18.0));
                                        });

                                        ui.add_space(12.0);

                                        // Right side: Replacement Key
                                        ui.vertical(|ui| {
                                            ui.label(
                                                RichText::new("Replacement / Combo")
                                                    .small()
                                                    .color(Color32::GRAY),
                                            );
                                            ui.add(
                                                egui::TextEdit::singleline(
                                                    &mut map.replacement_key,
                                                )
                                                .desired_width(180.0)
                                                .hint_text("e.g. B or Ctrl+C"),
                                            );
                                        });

                                        // Far right: Delete button
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.add_space(8.0);
                                                let del_btn = egui::Button::new(
                                                    RichText::new("🗑")
                                                        .size(16.0)
                                                        .color(Color32::from_rgb(255, 80, 80)),
                                                )
                                                .min_size(egui::vec2(32.0, 32.0))
                                                .corner_radius(6.0);

                                                if ui
                                                    .add(del_btn)
                                                    .on_hover_text("Delete mapping")
                                                    .clicked()
                                                {
                                                    to_remove = Some(idx);
                                                }
                                            },
                                        );
                                    });
                                });
                                ui.add_space(8.0);
                            }

                            // Clean up removed items outside the loop
                            if let Some(i) = to_remove {
                                state.mappings.remove(i);
                            }
                        }
                    });
            });
    });
}
