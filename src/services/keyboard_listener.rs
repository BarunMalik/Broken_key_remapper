#[cfg(target_os = "windows")]
mod platform {
    use std::collections::{HashMap, HashSet};
    use std::ffi::c_int;
    use std::sync::{
        atomic::{AtomicI32, Ordering},
        Mutex, OnceLock,
    };
    use std::time::{Duration, Instant};

    use crate::state::app_state::KeyMap;

    #[link(name = "keyboard_listener", kind = "static")]
    unsafe extern "C" {
        fn register_key_callback(cb: Option<extern "C" fn(c_int, c_int) -> c_int>);
        fn toggle_listener(enabled: c_int);
        fn press_key(vk: c_int);
        fn release_key(vk: c_int);
        fn tap_key(vk: c_int);
    }

    static CALLBACK_REGISTERED: OnceLock<()> = OnceLock::new();

    // Reverse trigger index:
    // key = replacement-combo canonical string, value = action to simulate.
    static REVERSE_MAP: OnceLock<Mutex<HashMap<String, ReverseAction>>> = OnceLock::new();

    // Live pressed keys (by vk) used to detect combo-down and combo-up.
    static PRESSED_KEYS: OnceLock<Mutex<HashSet<i32>>> = OnceLock::new();

    // Active hold combos -> broken key currently held down.
    static ACTIVE_HOLDS: OnceLock<Mutex<HashMap<String, i32>>> = OnceLock::new();

    // Capture mode:
    // -1 = idle
    // -2 = armed (recording)
    static CAPTURE_MODE: AtomicI32 = AtomicI32::new(-1);

    #[derive(Default)]
    struct ComboCaptureState {
        order: Vec<i32>,
        down: HashMap<i32, bool>,
        last_event_at: Option<Instant>,
    }

    static COMBO_CAPTURE: OnceLock<Mutex<ComboCaptureState>> = OnceLock::new();

    const CAPTURE_IDLE_TIMEOUT_MS: u64 = 300;

    #[derive(Clone, Debug)]
    struct ReverseAction {
        broken_vk: i32,
        tap_once: bool,
    }

    fn reverse_map() -> &'static Mutex<HashMap<String, ReverseAction>> {
        REVERSE_MAP.get_or_init(|| Mutex::new(HashMap::new()))
    }

    fn pressed_keys() -> &'static Mutex<HashSet<i32>> {
        PRESSED_KEYS.get_or_init(|| Mutex::new(HashSet::new()))
    }

    fn active_holds() -> &'static Mutex<HashMap<String, i32>> {
        ACTIVE_HOLDS.get_or_init(|| Mutex::new(HashMap::new()))
    }

    fn combo_capture() -> &'static Mutex<ComboCaptureState> {
        COMBO_CAPTURE.get_or_init(|| Mutex::new(ComboCaptureState::default()))
    }

    fn is_capture_active() -> bool {
        CAPTURE_MODE.load(Ordering::SeqCst) == -2
    }

    extern "C" fn key_event_callback(key: c_int, state: c_int) -> c_int {
        let vk = key as i32;
        let is_key_down = state == 1;

        println!(
            "[listener] key event: vk={} state={}",
            vk,
            if is_key_down { "down" } else { "up" }
        );

        // Focus-safe combo recording: swallow all keys while recording.
        if is_capture_active() {
            if let Ok(mut cap) = combo_capture().lock() {
                cap.last_event_at = Some(Instant::now());
                if is_key_down {
                    if !cap.down.get(&vk).copied().unwrap_or(false) {
                        cap.down.insert(vk, true);
                        if !cap.order.contains(&vk) {
                            cap.order.push(vk);
                        }
                    }
                } else {
                    cap.down.insert(vk, false);
                }
            }
            return 1;
        }

        // Update pressed key set.
        let current_keys = {
            let mut guard = match pressed_keys().lock() {
                Ok(g) => g,
                Err(_) => return 0,
            };

            if is_key_down {
                guard.insert(vk);
            } else {
                guard.remove(&vk);
            }

            guard.clone()
        };

        let combo_key = canonical_combo_from_set(&current_keys);

        // 1) Combo-down trigger (reverse mapping): replacement combo -> simulate broken key.
        if is_key_down && !combo_key.is_empty() {
            if let Ok(map_guard) = reverse_map().lock() {
                if let Some(action) = map_guard.get(&combo_key).cloned() {
                    if action.tap_once {
                        unsafe { tap_key(action.broken_vk as c_int) };
                    } else {
                        // Hold mode: key down on combo match; key up when combo breaks.
                        if let Ok(mut holds) = active_holds().lock() {
                            if !holds.contains_key(&combo_key) {
                                unsafe { press_key(action.broken_vk as c_int) };
                                holds.insert(combo_key.clone(), action.broken_vk);
                            }
                        }
                    }
                    return 1;
                }
            }
        }

        // 2) Hold release check: if any active hold combo is no longer satisfied, release broken key.
        // This runs on every event.
        if let (Ok(mut holds), Ok(map_guard)) = (active_holds().lock(), reverse_map().lock()) {
            let mut to_release: Vec<(String, i32)> = Vec::new();

            for (combo, broken_vk) in holds.iter() {
                let keys_required = parse_combo_to_vks(combo);
                let still_active = keys_required.iter().all(|k| current_keys.contains(k));

                // If combo no longer active OR mapping switched to tap_once/not found => release.
                let still_valid_hold_mapping = map_guard
                    .get(combo)
                    .map(|a| !a.tap_once && a.broken_vk == *broken_vk)
                    .unwrap_or(false);

                if !still_active || !still_valid_hold_mapping {
                    to_release.push((combo.clone(), *broken_vk));
                }
            }

            for (combo, broken_vk) in to_release {
                unsafe { release_key(broken_vk as c_int) };
                holds.remove(&combo);
            }
        }

        // Let event pass when no reverse mapping consumed it.
        0
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

        if !enabled {
            // Defensive cleanup for hold mode on disable.
            if let Ok(mut holds) = active_holds().lock() {
                for (_combo, broken_vk) in holds.drain() {
                    unsafe { release_key(broken_vk as c_int) };
                }
            }
            if let Ok(mut keys) = pressed_keys().lock() {
                keys.clear();
            }
        }

        unsafe {
            toggle_listener(if enabled { 1 } else { 0 });
        }
    }

    pub fn set_mappings(mappings: &[KeyMap]) {
        init();

        let mut new_map: HashMap<String, ReverseAction> = HashMap::new();

        for map in mappings {
            let Some(broken_vk) = parse_key(&map.broken_key) else {
                continue;
            };

            let replacement = parse_combo(&map.replacement_key);
            if replacement.is_empty() {
                continue;
            }

            let combo_key = canonical_combo_from_slice(&replacement);
            if combo_key.is_empty() {
                continue;
            }

            new_map.insert(
                combo_key,
                ReverseAction {
                    broken_vk,
                    tap_once: map.tap_once,
                },
            );
        }

        if let Ok(mut guard) = reverse_map().lock() {
            *guard = new_map;
        }
    }

    pub fn begin_key_capture() {
        init();
        CAPTURE_MODE.store(-2, Ordering::SeqCst);
        if let Ok(mut cap) = combo_capture().lock() {
            cap.order.clear();
            cap.down.clear();
            cap.last_event_at = None;
        }
        println!("[listener] combo capture started");
    }

    pub fn cancel_key_capture() {
        CAPTURE_MODE.store(-1, Ordering::SeqCst);
        if let Ok(mut cap) = combo_capture().lock() {
            cap.order.clear();
            cap.down.clear();
            cap.last_event_at = None;
        }
        println!("[listener] combo capture cancelled");
    }

    pub fn poll_captured_combo_label() -> Option<String> {
        if !is_capture_active() {
            return None;
        }

        let mut should_finish = false;
        let mut label = String::new();

        if let Ok(mut cap) = combo_capture().lock() {
            let Some(last) = cap.last_event_at else {
                return None;
            };

            if last.elapsed() >= Duration::from_millis(CAPTURE_IDLE_TIMEOUT_MS)
                && !cap.order.is_empty()
            {
                let parts: Vec<String> = cap.order.iter().map(|&vk| vk_to_label(vk)).collect();
                label = parts.join("+");
                should_finish = true;

                cap.order.clear();
                cap.down.clear();
                cap.last_event_at = None;
            }
        }

        if should_finish {
            CAPTURE_MODE.store(-1, Ordering::SeqCst);
            println!("[listener] combo capture finished: {}", label);
            return Some(label);
        }

        None
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

    fn canonical_combo_from_set(keys: &HashSet<i32>) -> String {
        let v: Vec<i32> = keys.iter().copied().collect();
        canonical_combo_from_slice(&v)
    }

    fn canonical_combo_from_slice(keys: &[i32]) -> String {
        if keys.is_empty() {
            return String::new();
        }

        let mut uniq: Vec<i32> = keys.to_vec();
        uniq.sort_unstable();
        uniq.dedup();

        // deterministic modifier-first ordering
        uniq.sort_by_key(|k| sort_rank(*k));

        uniq.into_iter()
            .map(vk_to_label)
            .collect::<Vec<_>>()
            .join("+")
    }

    fn parse_combo_to_vks(combo: &str) -> Vec<i32> {
        parse_combo(combo)
    }

    fn sort_rank(vk: i32) -> i32 {
        match vk {
            0x11 => 0,        // Ctrl
            0x10 => 1,        // Shift
            0x12 => 2,        // Alt
            0x5B | 0x5C => 3, // Win
            _ => 10_000 + vk,
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
    pub fn poll_captured_combo_label() -> Option<String> {
        None
    }
}

pub use platform::{
    begin_key_capture, cancel_key_capture, poll_captured_combo_label, set_enabled, set_mappings,
};
