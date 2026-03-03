use crate::state::app_state::AppState;
use crate::ui;
use eframe::egui;

pub struct MyApp {
    pub state: AppState,
}

impl MyApp {
    // We change the signature to accept 'state'
    pub fn new(_cc: &eframe::CreationContext<'_>, state: AppState) -> Self {
        setup_custom_fonts(&_cc.egui_ctx);
        Self {
            state, // Use the state passed from main.rs instead of default()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("🏠 Home").clicked() {
                    self.state.current_screen = "home".into();
                }
                if ui.button("⚙ Settings").clicked() {
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

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "open_sans".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/open.ttf")).into(),
    );

    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "open_sans".to_owned());

    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "open_sans".to_owned());

    ctx.set_fonts(fonts);
}
