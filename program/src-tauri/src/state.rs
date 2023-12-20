use std::sync::RwLock;

use once_cell::sync::OnceCell;

use crate::settings::Settings;

#[derive(Default)]
pub struct AppState {
    pub settings: OnceCell<RwLock<Settings>>,
}
