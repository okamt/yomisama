use std::{
    sync::{Arc, Mutex},
    thread,
};

use device_query::{DeviceEvents, DeviceState};

fn main() {
    let device_state = DeviceState::new();
    let previous = Arc::new(Mutex::new(<Option<String>>::None));

    let guard = device_state.on_mouse_up(move |button| {
        if button == &1 {
            let text = selection::get_text();
            let mut previous_data = previous.lock().unwrap();
            if previous_data.as_ref().is_some_and(|t| t != &text) {
                println!("{:?}", text);
                *previous_data = Some(text);
            } else if previous_data.is_none() {
                *previous_data = Some(text);
            }
        }
    });

    loop {
        thread::park()
    }
}
