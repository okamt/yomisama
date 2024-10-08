//! Window creation and management module.
//!
//! Since Tauri has no built-in functionality for passing data to a newly created window, this module implements its own system:
//! - Window is created through [`tauri::WindowBuilder`], its label set to an unique ID (auto increment [`AtomicU64`])
//! - A [`Payload`] associated to the ID is inserted into `PAYLOAD_QUEUE`.
//! - The base frontend component `App.svelte` invokes [`window_loaded`] when ready.
//! - [`window_loaded`] fetches the associated [`Payload`] and returns it, but doesn't remove it from `PAYLOAD_QUEUE` (since that would break hot reloading).
//! - When a window requests to be closed, `App.svelte` invokes [`window_unloading`], which finally deletes the payload.

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tauri::{Manager, Runtime, Window};
use typescript_type_def::TypeDef;

use crate::config::DEFAULT_CONFIG_DIRECTORY_PATH;

static NEXT_WINDOW_ID: AtomicU64 = AtomicU64::new(0);
static PAYLOAD_QUEUE: Lazy<Mutex<HashMap<u64, Payload>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn get_next_window_id() -> u64 {
    let id = NEXT_WINDOW_ID.load(Ordering::SeqCst);
    let result = id;
    NEXT_WINDOW_ID.store(id + 1, Ordering::SeqCst);
    result
}

fn get_window_label() -> String {
    get_next_window_id().to_string()
}

/// Payload data to be sent to newly created windows.
///
/// Each variant must correspond to the name of a Svelte component.
#[derive(Debug, Serialize, Deserialize, Clone, TypeDef)]
#[serde(tag = "component")]
pub enum Payload {
    #[serde(rename_all = "camelCase")]
    FirstTimeSetup {
        default_config_dir: String,
    },
    Query,
}

fn queue_payload(window: &Window<impl Runtime>, payload: Payload) {
    let id: u64 = window
        .label()
        .parse()
        .expect("window label should be integer");
    let mut queue = PAYLOAD_QUEUE.lock().unwrap();
    queue.insert(id, payload);
}

#[tauri::command]
pub fn window_loaded(window: Window) -> Payload {
    let queue = PAYLOAD_QUEUE.lock().unwrap();
    let id = str::parse(window.label()).expect("window label should be integer");
    queue.get(&id).expect("no payload found in queue").clone()
}

#[tauri::command]
pub fn window_unloading(window: Window) {
    let mut queue = PAYLOAD_QUEUE.lock().unwrap();
    let id = str::parse(window.label()).expect("window label should be integer");
    queue.remove(&id);
}

pub fn spawn_first_time_setup_window<M: Manager<R>, R: Runtime>(
    app: &M,
) -> tauri::Result<Window<R>> {
    let window = tauri::WindowBuilder::new(
        app,
        get_window_label(),
        tauri::WindowUrl::App("index.html".into()),
    )
    .fullscreen(false)
    .resizable(false)
    .title("Yomisama - First time setup")
    .inner_size(500.0, 500.0)
    .center()
    .decorations(false)
    .build()?;

    queue_payload(
        &window,
        Payload::FirstTimeSetup {
            default_config_dir: DEFAULT_CONFIG_DIRECTORY_PATH.clone(),
        },
    );

    Ok(window)
}
