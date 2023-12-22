//! Tauri app state module.

use std::sync::RwLock;

use once_cell::sync::OnceCell;

use crate::config::Config;

/// App state for Tauri.
pub struct AppState {
    pub config: OnceCell<RwLock<Config>>,
}
