#[cfg(target_os = "windows")]
mod platform {
    use std::collections::HashMap;
    use std::ffi::c_int;
    use std::sync::{
        atomic::{AtomicI32, Ordering},
        Mutex, OnceLock,
    };

    use crate::state::app_state::KeyMap;

    #[link(name = "keyboard_listener", kind = "static")]
    unsafe extern "C" {
        fn register_key_callback(cb: Option<extern "C" fn(c_int, c_int) -> c_int>);
        fn toggle_listener(enabled: c_int);
        fn press_key(vk: c_int);
        fn release_key(vk: c_int);
    }

    static REMAP_TABLE: OnceLock<Mutex<HashMap<i32, Vec<i32>>>> = OnceLock::new();
    static CALLBACK_REGISTERED: OnceLock<()> = OnceLock::new();

    // UI key-capture state (for "record key into textbox")
    static CAPTURE_SLOT: AtomicI32 = AtomicI32::new(-1);

    fn remap_table() -> &'static Mutex<HashMap<i32, Vec<i32>>> {
        REMAP_TABLE.get_or_init(|| Mutex::new(HashMap::new()))
    }

    extern "C" fn key_event_callback(key: c_int, state: c_int) -> c_int {
        let key = key as i32;
        let is_key_down = state == 1;

        println!(
            "[listener] key event: vk={} state={}",
            key,
            if is_key_down { "down" } else { "up" }
        );

        // Capture mode: on key-down, store vk for UI and swallow the key event.
        if is_key_down {
            let cur = CAPTURE_SLOT.load(Ordering::SeqCst);
            if cur == -2 {
                CAPTURE_SLOT.store(key, Ordering::SeqCst);
                return 1;
            }
        }

        let table_guard = match remap_table().lock() {
            Ok(g) => g,
            Err(_) => return 0,
        };

        let Some(targets) = table_guard.get(&key) else {
            return 0;
        };

        if targets.is_empty() {
            return 0;
        }

        if is_key_down {
            for &vk in targets {
                unsafe { press_key(vk as c_int) };
            }
        } else {
            for &vk in targets.iter().rev() {
                unsafe { release_key(vk as c_int) };
            }
        }

        // Block original key event when remapping happened.
        1
    }

    pub fn init() {
        CALLBACK_REGISTERED.get_or_init(|| unsafe {
            register_key_callback(Some(key_event_callback));
        });
    }

    pub fn set_enabled(enabled: bool) {
        init();
        println!(
            "[listener] {}",
            if enabled { "enabled" } else { "disabled" }
        );
        unsafe {
            toggle_listener(if enabled { 1 } else { 0 });
        }
    }

    pub fn set_mappings(mappings: &[KeyMap]) {
        init();

        let mut new_table: HashMap<i32, Vec<i32>> = HashMap::new();

        for map in mappings {
            let Some(src) = parse_key(&map.broken_key) else {
                continue;
            };
            let dst = parse_combo(&map.replacement_key);
            if dst.is_empty() {
                continue;
            }

            new_table.insert(src, dst);
        }

        if let Ok(mut guard) = remap_table().lock() {
            *guard = new_table;
        }
    }

    pub fn begin_key_capture() {
        init();
        CAPTURE_SLOT.store(-2, Ordering::SeqCst); // armed
    }

    pub fn cancel_key_capture() {
        CAPTURE_SLOT.store(-1, Ordering::SeqCst);
    }

    pub fn poll_captured_vk() -> Option<i32> {
        let v = CAPTURE_SLOT.load(Ordering::SeqCst);
        if v >= 0 {
            CAPTURE_SLOT.store(-1, Ordering::SeqCst);
            Some(v)
        } else {
            None
        }
    }

    pub fn vk_to_label(vk: i32) -> String {
        if (0x41..=0x5A).contains(&vk) || (0x30..=0x39).contains(&vk) {
            return (vk as u8 as char).to_string();
        }

        match vk {
            0x11 => "Ctrl".into(),
            0x10 => "Shift".into(),
            0x12 => "Alt".into(),
            0x5B => "LWin".into(),
            0x5C => "RWin".into(),

            0x20 => "Space".into(),
            0x09 => "Tab".into(),
            0x0D => "Enter".into(),
            0x1B => "Esc".into(),
            0x08 => "Backspace".into(),
            0x14 => "CapsLock".into(),

            0x26 => "Up".into(),
            0x28 => "Down".into(),
            0x25 => "Left".into(),
            0x27 => "Right".into(),

            0x24 => "Home".into(),
            0x23 => "End".into(),
            0x21 => "PageUp".into(),
            0x22 => "PageDown".into(),
            0x2D => "Insert".into(),
            0x2E => "Delete".into(),

            0x70 => "F1".into(),
            0x71 => "F2".into(),
            0x72 => "F3".into(),
            0x73 => "F4".into(),
            0x74 => "F5".into(),
            0x75 => "F6".into(),
            0x76 => "F7".into(),
            0x77 => "F8".into(),
            0x78 => "F9".into(),
            0x79 => "F10".into(),
            0x7A => "F11".into(),
            0x7B => "F12".into(),

            _ => format!("VK_{vk}"),
        }
    }

    fn parse_combo(input: &str) -> Vec<i32> {
        let separators = ['+', ' ', ','];
        input
            .split(|c| separators.contains(&c))
            .filter_map(|part| {
                let t = part.trim();
                if t.is_empty() {
                    None
                } else {
                    parse_key(t)
                }
            })
            .collect()
    }

    fn parse_key(raw: &str) -> Option<i32> {
        let key = raw.trim().to_ascii_uppercase();
        if key.is_empty() {
            return None;
        }

        if key.len() == 1 {
            let b = key.as_bytes()[0];
            if b.is_ascii_uppercase() || b.is_ascii_digit() {
                return Some(b as i32);
            }
        }

        match key.as_str() {
            "CTRL" | "CONTROL" => Some(0x11),
            "SHIFT" => Some(0x10),
            "ALT" | "MENU" => Some(0x12),
            "WIN" | "LWIN" | "META" => Some(0x5B),
            "RWIN" => Some(0x5C),

            "SPACE" => Some(0x20),
            "TAB" => Some(0x09),
            "ENTER" | "RETURN" => Some(0x0D),
            "ESC" | "ESCAPE" => Some(0x1B),
            "BACKSPACE" | "BS" => Some(0x08),
            "CAPSLOCK" => Some(0x14),

            "UP" => Some(0x26),
            "DOWN" => Some(0x28),
            "LEFT" => Some(0x25),
            "RIGHT" => Some(0x27),

            "HOME" => Some(0x24),
            "END" => Some(0x23),
            "PGUP" | "PAGEUP" => Some(0x21),
            "PGDN" | "PAGEDOWN" => Some(0x22),
            "INSERT" | "INS" => Some(0x2D),
            "DELETE" | "DEL" => Some(0x2E),

            "F1" => Some(0x70),
            "F2" => Some(0x71),
            "F3" => Some(0x72),
            "F4" => Some(0x73),
            "F5" => Some(0x74),
            "F6" => Some(0x75),
            "F7" => Some(0x76),
            "F8" => Some(0x77),
            "F9" => Some(0x78),
            "F10" => Some(0x79),
            "F11" => Some(0x7A),
            "F12" => Some(0x7B),

            _ => None,
        }
    }
}

#[cfg(not(target_os = "windows"))]
mod platform {
    use crate::state::app_state::KeyMap;

    pub fn set_enabled(_enabled: bool) {}
    pub fn set_mappings(_mappings: &[KeyMap]) {}

    pub fn begin_key_capture() {}
    pub fn cancel_key_capture() {}
    pub fn poll_captured_vk() -> Option<i32> {
        None
    }
    pub fn vk_to_label(vk: i32) -> String {
        format!("VK_{vk}")
    }
}

pub use platform::{
    begin_key_capture, cancel_key_capture, poll_captured_vk, set_enabled, set_mappings, vk_to_label,
};
