use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

pub fn init_tray<FShow, FExit>(on_show_window: FShow, on_exit: FExit) -> TrayIcon
where
    FShow: Fn() + Send + 'static,
    FExit: Fn() + Send + 'static,
{
    let tray_menu = Menu::new();
    let show_item = MenuItem::new("Show Window", true, None);
    let exit_item = MenuItem::new("Exit", true, None);

    let show_id = show_item.id().clone();
    let exit_id = exit_item.id().clone();

    tray_menu.append_items(&[&show_item, &exit_item]).unwrap();

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Bekar Keyboard")
        .with_icon(make_demo_icon())
        .build()
        .unwrap();

    // Background thread: blocks on MenuEvent and directly invokes callbacks
    std::thread::spawn(move || {
        let menu_channel = MenuEvent::receiver();
        loop {
            if let Ok(event) = menu_channel.recv() {
                if event.id == show_id {
                    on_show_window();
                } else if event.id == exit_id {
                    on_exit();
                }
            }
        }
    });

    tray_icon
}

/// Generates a simple 32x32 solid-blue RGBA icon at runtime.
/// Replace this with a real icon later.
fn make_demo_icon() -> Icon {
    const SIZE: u32 = 32;
    // Windows-blue RGBA: #0078D7 FF
    let rgba: Vec<u8> = (0..SIZE * SIZE)
        .flat_map(|_| [0u8, 120, 215, 255])
        .collect();
    Icon::from_rgba(rgba, SIZE, SIZE).unwrap()
}
