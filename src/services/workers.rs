use crate::state::{
    self,
    app_state::{AppState, KeyMap},
};
use rdev::{listen, Event, EventType};
use std::sync::mpsc;
use std::thread;

//this is our bool state.listener_enabled

/// Starts listening to keyboard events in a background thread
/// Returns a receiver channel that gets Event objects
pub fn start_listener() -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            // Send events through the channel
            let _ = tx.send(event);
        }) {
            eprintln!("Error starting listener: {:?}", error);
        }
    });

    rx
}

/// Process a single event from the listener
pub fn callback(event: Event) {
    

   match event.event_type {
        // We only care about KeyPress. We ignore KeyRelease and Mouse events.
        EventType::KeyPress(key) => {
            println!("Key pressed: {:?}", key);
        }
        // The underscore (_) means "everything else"
        // We do nothing for these events.
        _ => {} 
    }
}
