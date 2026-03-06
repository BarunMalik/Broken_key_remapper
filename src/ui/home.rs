use crate::services::keyboard_listener;
use crate::state::app_state::{AppState, KeyMap};
use eframe::egui::{self, Color32, RichText};

pub fn show(ctx: &egui::Context, state: &mut AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::Frame::NONE
            .inner_margin(egui::Margin {
                left: 16,
                right: 16,
                top: 16,
                bottom: 16,
            })
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("⌨ Key Remapper").size(28.0).strong());

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

                ui.horizontal(|ui| {
                    ui.label(RichText::new("Your Mappings").size(18.0).strong());

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let add_btn = egui::Button::new(RichText::new("➕ Add Mapping").strong())
                            .min_size(egui::vec2(120.0, 28.0))
                            .corner_radius(6.0);

                        if ui.add(add_btn).clicked() {
                            state.mappings.push(KeyMap {
                                broken_key: "".to_string(),
                                replacement_key: "".to_string(),
                                tap_once: true,
                            });
                        }
                    });
                });

                ui.add_space(16.0);

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        if state.mappings.is_empty() {
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
                                    ui.set_width(ui.available_width());

                                    ui.horizontal(|ui| {
                                        ui.vertical(|ui| {
                                            ui.label(
                                                RichText::new("Broken Key")
                                                    .small()
                                                    .color(Color32::GRAY),
                                            );

                                            ui.horizontal(|ui| {
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut map.broken_key)
                                                        .desired_width(140.0)
                                                        .hint_text("e.g. A"),
                                                );

                                                let broken_recording =
                                                    state.mapping_record_target == Some((idx, false));

                                                let broken_record_label = if broken_recording {
                                                    "⏺ Recording..."
                                                } else {
                                                    "⏺ Record"
                                                };

                                                if ui
                                                    .add(
                                                        egui::Button::new(
                                                            RichText::new(broken_record_label)
                                                                .small(),
                                                        )
                                                        .corner_radius(6.0),
                                                    )
                                                    .on_hover_text(
                                                        "Click and press a key for Broken Key",
                                                    )
                                                    .clicked()
                                                {
                                                    if broken_recording {
                                                        keyboard_listener::cancel_key_capture();
                                                        state.mapping_record_target = None;

                                                        if state.listener_restore_after_recording {
                                                            state.listener_enabled = true;
                                                            state.listener_restore_after_recording =
                                                                false;
                                                        }
                                                    } else {
                                                        state.listener_restore_after_recording =
                                                            state.listener_enabled;
                                                        if state.listener_enabled {
                                                            state.listener_enabled = false;
                                                        }

                                                        state.mapping_record_target =
                                                            Some((idx, false));
                                                        keyboard_listener::begin_key_capture();
                                                    }
                                                }
                                            });
                                        });

                                        ui.add_space(12.0);

                                        ui.vertical(|ui| {
                                            ui.add_space(16.0);
                                            ui.label(RichText::new("➡").size(18.0));
                                        });

                                        ui.add_space(12.0);

                                        ui.vertical(|ui| {
                                            ui.label(
                                                RichText::new("Replacement / Combo")
                                                    .small()
                                                    .color(Color32::GRAY),
                                            );

                                            ui.horizontal(|ui| {
                                                ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut map.replacement_key,
                                                    )
                                                    .desired_width(180.0)
                                                    .hint_text("e.g. B or Ctrl+C"),
                                                );

                                                let repl_recording =
                                                    state.mapping_record_target == Some((idx, true));

                                                let repl_record_label = if repl_recording {
                                                    "⏺ Recording..."
                                                } else {
                                                    "⏺ Record"
                                                };

                                                if ui
                                                    .add(
                                                        egui::Button::new(
                                                            RichText::new(repl_record_label).small(),
                                                        )
                                                        .corner_radius(6.0),
                                                    )
                                                    .on_hover_text(
                                                        "Click and press a key for Replacement",
                                                    )
                                                    .clicked()
                                                {
                                                    if repl_recording {
                                                        keyboard_listener::cancel_key_capture();
                                                        state.mapping_record_target = None;

                                                        if state.listener_restore_after_recording {
                                                            state.listener_enabled = true;
                                                            state.listener_restore_after_recording =
                                                                false;
                                                        }
                                                    } else {
                                                        state.listener_restore_after_recording =
                                                            state.listener_enabled;
                                                        if state.listener_enabled {
                                                            state.listener_enabled = false;
                                                        }

                                                        state.mapping_record_target =
                                                            Some((idx, true));
                                                        keyboard_listener::begin_key_capture();
                                                    }
                                                }
                                            });
                                        });

                                        ui.add_space(12.0);

                                        ui.vertical(|ui| {
                                            ui.label(
                                                RichText::new("Behavior")
                                                    .small()
                                                    .color(Color32::GRAY),
                                            );

                                            ui.checkbox(
                                                &mut map.tap_once,
                                                "Tap once (otherwise hold while combo held)",
                                            )
                                            .on_hover_text(
                                                "When checked: trigger broken key once on combo.\nWhen unchecked: mirror key down/up.",
                                            );
                                        });

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

                            if let Some(i) = to_remove {
                                if state
                                    .mapping_record_target
                                    .map(|(record_idx, _)| record_idx == i)
                                    .unwrap_or(false)
                                {
                                    keyboard_listener::cancel_key_capture();
                                    state.mapping_record_target = None;

                                    if state.listener_restore_after_recording {
                                        state.listener_enabled = true;
                                        state.listener_restore_after_recording = false;
                                    }
                                }

                                state.mappings.remove(i);

                                if let Some((record_idx, field)) = state.mapping_record_target {
                                    if record_idx > i {
                                        state.mapping_record_target = Some((record_idx - 1, field));
                                    }
                                }
                            }
                        }
                    });
            });
    });
}
