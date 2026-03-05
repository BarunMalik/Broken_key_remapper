use crate::services::system_tray::init_tray;
use crate::state::app_state::AppState;
use crate::state::helper::was_auto_launched;
use crate::ui;
use eframe::egui;
use tray_icon::TrayIcon;

pub struct MyApp {
    pub state: AppState,
    tray: Option<TrayIcon>,
    /// True only on the very first frame — used to minimize if auto-launched.
    first_frame: bool,
}

impl MyApp {
    fn create_tray(egui_ctx: egui::Context, with_logs: bool) -> TrayIcon {
        init_tray(
            move || {
                if with_logs {
                    println!("Showing!");
                }
                //egui_ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                egui_ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
                egui_ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                egui_ctx.request_repaint();
            },
            move || {
                if with_logs {
                    println!("exiting!");
                }
                std::process::exit(0);
            },
        )
    }

    pub fn new(cc: &eframe::CreationContext<'_>, state: AppState) -> Self {
        setup_custom_fonts(&cc.egui_ctx);

        let start_minimized = was_auto_launched();

        if state.task_bar {
            let tray = Self::create_tray(cc.egui_ctx.clone(), false);
            Self {
                state,
                tray: Some(tray),
                first_frame: start_minimized,
            }
        } else {
            Self {
                state,
                tray: None,
                first_frame: start_minimized,
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- Auto-launch: minimize on first frame ---
        if self.first_frame {
            self.first_frame = false;
            #[cfg(target_os = "windows")]
            {
                ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            }
        }

        // --- Minimize Instead of Close ---
        if self.state.run_in_background {
            if ctx.input(|i| i.viewport().close_requested()) {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));

                //this is a fix for now (temp fix)
                //ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
            }
        }

        // --- Dynamic Tray Toggle ---
        if self.state.task_bar && self.tray.is_none() {
            let tray = Self::create_tray(ctx.clone(), false);
            self.tray = Some(tray);
        }

        if !self.state.task_bar && self.tray.is_some() {
            self.tray = None;
        }

        // --- UI ---
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                ui.spacing_mut().item_spacing.x = 16.0;

                if ui
                    .selectable_label(
                        self.state.current_screen == "home",
                        egui::RichText::new("🏠 Home").size(16.0),
                    )
                    .clicked()
                {
                    self.state.current_screen = "home".into();
                }

                if ui
                    .selectable_label(
                        self.state.current_screen == "settings",
                        egui::RichText::new("⚙ Settings").size(16.0),
                    )
                    .clicked()
                {
                    self.state.current_screen = "settings".into();
                }
            });
            ui.add_space(8.0);
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
