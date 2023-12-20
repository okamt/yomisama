use std::path::{Path, PathBuf};

use api::database::{dictionary::cdb::CDBDictionary, Database};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub static DEFAULT_CONFIG_DIRECTORY_PATH: Lazy<String> = Lazy::new(|| {
    dirs::config_dir()
        .expect("default config directory path doesn't exist (unsupported OS?)")
        .join("yomisama")
        .to_str()
        .expect("default config directory path should be valid UTF-8")
        .to_owned()
});

pub static DEFAULT_CONFIG_FILE_PATH: Lazy<String> = Lazy::new(|| {
    dirs::config_dir()
        .expect("default config directory path doesn't exist (unsupported OS?)")
        .join("yomisama/config.json")
        .to_str()
        .expect("default config directory path should be valid UTF-8")
        .to_owned()
});

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub database: Database<CDBDictionary>,
    #[serde(skip)]
    path: PathBuf,
}

impl Settings {
    fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Self, SettingsFileReadError> {
        let mut result = serde_json::from_slice::<Self>(&std::fs::read(&path)?)
            .map_err(SettingsFileReadError::Deserialize);

        if let Ok(ref mut settings) = result {
            settings.path = path.as_ref().to_path_buf();
        }

        result
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

#[derive(Serialize, Deserialize)]
pub enum SettingsFile {
    Settings(Settings),
    Redirect(PathBuf),
}

impl SettingsFile {
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Self, SettingsFileReadError> {
        serde_json::from_slice(&std::fs::read(path)?).map_err(SettingsFileReadError::Deserialize)
    }

    pub fn into_settings(self) -> Result<Settings, SettingsFileReadError> {
        match self {
            Self::Settings(s) => Ok(s),
            Self::Redirect(path) => Settings::read_from_file(path),
        }
    }
}

#[derive(Debug, Error)]
pub enum SettingsFileReadError {
    #[error("settings file IO error")]
    Io(#[from] std::io::Error),
    #[error("settings file deserialization error")]
    Deserialize(#[from] serde_json::Error),
}
