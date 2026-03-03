use eframe::egui;

use crate::state::app_state::AppState;
use crate::ui;

pub struct MyApp {
    pub state: AppState,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: AppState::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Home").clicked() {
                    self.state.current_screen = "home".into();
                }
                if ui.button("Settings").clicked() {
                    self.state.current_screen = "settings".into();
                }
            });
        });

        match self.state.current_screen.as_str() {
            "settings" => ui::settings::show(ctx, &mut self.state),
            _ => ui::home::show(ctx, &mut self.state),
        }
    }
}
