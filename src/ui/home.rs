use crate::state::app_state::{AppState, KeyMap};
use eframe::egui;

pub fn show(ctx: &egui::Context, state: &mut AppState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Key Remapper");
  ui.checkbox(&mut state.listener_enabled, "Listener Enabled");
        ui.separator();

        // --- Header / Add Button ---
        ui.horizontal(|ui| {
            ui.label("Configure your keys:");
            if ui.button("➕ Add Key").clicked() {
                state.mappings.push(KeyMap {
                    broken_key: "".to_string(),
                    replacement_key: "".to_string(),
                });
            }
        });

        ui.add_space(10.0);

        // --- Mappings List ---
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut to_remove = None;

            for (idx, map) in state.mappings.iter_mut().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Broken Key:");
                            ui.text_edit_singleline(&mut map.broken_key);
                        });

                        ui.label("➡");

                        ui.vertical(|ui| {
                            ui.label("Replacement / Combo:");
                            ui.text_edit_singleline(&mut map.replacement_key);
                        });

                        // Delete button for this specific mapping
                        if ui.button("🗑").clicked() {
                            to_remove = Some(idx);
                        }
                    });
                });
                ui.add_space(5.0);
            }

            // Clean up removed items
            if let Some(i) = to_remove {
                state.mappings.remove(i);
            }
        });
    });
}