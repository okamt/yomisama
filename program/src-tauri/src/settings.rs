use std::path::PathBuf;

use api::database::{dictionary::cdb::CDBDictionary, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub database: Database<CDBDictionary>,
}

#[derive(Serialize, Deserialize)]
pub enum SettingsFile {
    Settings(Settings),
    Redirect(PathBuf),
}
