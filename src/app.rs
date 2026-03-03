use crate::state::app_state::AppState;
use crate::ui;
use eframe::egui;

use tray_icon::{
    TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuItem, MenuEvent, MenuId},
};

pub struct MyApp {
    pub state: AppState,
    tray: Option<TrayIcon>,
    show_id: Option<MenuId>,
    quit_id: Option<MenuId>,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>, state: AppState) -> Self {
        setup_custom_fonts(&cc.egui_ctx);

        if state.task_bar {
            let (tray, show_id, quit_id) = create_tray();
            Self {
                state,
                tray: Some(tray),
                show_id: Some(show_id),
                quit_id: Some(quit_id),
            }
        } else {
            Self {
                state,
                tray: None,
                show_id: None,
                quit_id: None,
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // --- Minimize Instead of Close ---
        if self.state.run_in_background {
            if ctx.input(|i| i.viewport().close_requested()) {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
            
            }
        }

        // --- Handle Tray Menu Events ---
        if let Ok(event) = MenuEvent::receiver().try_recv() {

            if Some(event.id.clone()) == self.show_id {
                
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));

            }

            if Some(event.id) == self.quit_id {
                std::process::exit(0);
            }
        }

        // --- Dynamic Tray Toggle ---
        if self.state.task_bar && self.tray.is_none() {
            let (tray, show_id, quit_id) = create_tray();
            self.tray = Some(tray);
            self.show_id = Some(show_id);
            self.quit_id = Some(quit_id);
        }

        if !self.state.task_bar && self.tray.is_some() {
            self.tray = None;
            self.show_id = None;
            self.quit_id = None;
        }

        // --- UI ---
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

fn create_tray() -> (TrayIcon, MenuId, MenuId) {
    let menu = Menu::new();

    let show = MenuItem::new("Show", true, None);
    let quit = MenuItem::new("Quit", true, None);

    let show_id = show.id().clone();
    let quit_id = quit.id().clone();

    menu.append(&show).unwrap();
    menu.append(&quit).unwrap();

    let tray = TrayIconBuilder::new()
        .with_tooltip("Broken Remapper Running")
        .with_menu(Box::new(menu))
        .build()
        .unwrap();

    (tray, show_id, quit_id)
}