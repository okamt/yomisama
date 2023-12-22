use std::sync::RwLock;

use once_cell::sync::OnceCell;

use crate::config::Config;

pub struct AppState {
    pub config: OnceCell<RwLock<Config>>,
}
