//! Tauri commands module.

use std::{path::PathBuf, sync::RwLock};

use thiserror::Error;

use crate::{
    config::{Config, ConfigFilePath, ConfigFileWriteError, CONFIG_FILE_NAME},
    state::AppState,
};

/// Sets up the app configuration if it's not already set up.
///
/// If `path` is None, the default configuration directory path is used.
#[tauri::command(rename_all = "snake_case")]
pub fn set_config_dir(path: Option<String>, state: tauri::State<AppState>) -> Result<(), Error> {
    let config_path = match path {
        Some(p) => {
            let mut config_pathbuf = PathBuf::from(p);

            let metadata =
                std::fs::metadata(config_pathbuf.as_path()).map_err(Error::ConfigDirIo)?;
            // Should never happen, but check anyways.
            if !metadata.is_dir() {
                return Err(Error::ConfigDirNotADir);
            }
            config_pathbuf.push(CONFIG_FILE_NAME);

            ConfigFilePath::Custom(config_pathbuf)
        }
        None => ConfigFilePath::Default,
    };

    let config = Config::new(config_path)?;
    state.config.get_or_init(|| RwLock::new(config));

    Ok(())
}

/// An error that gets sent back to the frontend.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ConfigFileWriteError(#[from] ConfigFileWriteError),
    #[error("config dir IO error: {}", .0)]
    ConfigDirIo(#[source] std::io::Error),
    #[error("config dir path is not a directory")]
    ConfigDirNotADir,
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
