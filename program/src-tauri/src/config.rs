//! Configuration and settings module.

use std::path::{Path, PathBuf};

use api::database::{dictionary::cdb::CDBDictionary, Database};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sob::Sob;
use thiserror::Error;
use typescript_type_def::TypeDef;

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
        .expect("default config file path should be valid UTF-8")
        .to_owned()
});

pub const CONFIG_FILE_NAME: &str = "config.json";

/// Configuration data.
///
/// Represents data being stored in the configuration directory, such as settings and dictionaries.
/// Can only be obtained through [`Config::new()`] or [`Config::read()`].
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub database: Database<CDBDictionary>,
    // This shouldn't be serialized because if a [`Config`] is being deserialized, we obviously know where the file is.
    // A [`Config`] will only ever be deserialized from [`Config::read()`].
    #[serde(skip)]
    path: ConfigFilePath,
}

/// Settings which can be changed by the end user.
///
/// Frontend code has direct access to this type so it should be kept simple in structure.
#[derive(Serialize, Deserialize, Default, TypeDef)]
pub struct Settings {}

impl Config {
    /// Creates a new configuration file, overwriting any existing file.
    pub fn new(path: ConfigFilePath) -> Result<Self, ConfigFileWriteError> {
        let config = Self {
            settings: Default::default(),
            database: Database::new(),
            path,
        };
        config.write()?;
        Ok(config)
    }

    /// Checks if the default configuration file path exists.
    pub fn exists() -> bool {
        ConfigFilePath::Default.as_path().exists()
    }

    /// Reads configuration from the appropriate file.
    pub fn read() -> Result<Self, ConfigFileReadError> {
        let config_file: ConfigFile =
            serde_json::from_slice(&std::fs::read(ConfigFilePath::Default.as_path())?)
                .map_err(ConfigFileReadError::Deserialize)?;

        let Sob::Owned(config) = (match config_file {
            ConfigFile::Config(config) => config,
            ConfigFile::Redirect(actual_path) => {
                let actual_config_file: ConfigFile =
                    serde_json::from_slice(&std::fs::read(actual_path.as_path())?)
                        .map_err(ConfigFileReadError::Deserialize)?;

                match actual_config_file {
                    ConfigFile::Config(config) => config,
                    ConfigFile::Redirect(_) => return Err(ConfigFileReadError::TooManyRedirects),
                }
            }
        }) else {
            // A deserialized `Sob` will always be `Sob::Owned`.
            unreachable!();
        };

        Ok(config)
    }

    /// Gets the [`ConfigFilePath`].
    pub fn get_path(&self) -> &ConfigFilePath {
        &self.path
    }

    /// Writes the configuration file, overwriting any existing file.
    pub fn write(&self) -> Result<(), ConfigFileWriteError> {
        let default_config_dir = Path::new(DEFAULT_CONFIG_DIRECTORY_PATH.as_str());
        if !default_config_dir.exists() {
            std::fs::create_dir(default_config_dir).map_err(ConfigFileWriteError::Io)?;
        }

        let config_file = ConfigFile::Config(Sob::Borrowed(&self));
        let data = serde_json::to_vec(&config_file).map_err(ConfigFileWriteError::Serialize)?;
        std::fs::write(self.path.as_path(), data).map_err(ConfigFileWriteError::Io)?;

        // If using a custom path, keep track of it by writing a redirect at the default path.
        if let ConfigFilePath::Custom(ref actual_path) = self.path {
            let redirect_config_file = ConfigFile::Redirect(actual_path.clone());
            let redirect_data = serde_json::to_vec(&redirect_config_file)
                .map_err(ConfigFileWriteError::Serialize)?;
            std::fs::write(ConfigFilePath::Default.as_path(), redirect_data)
                .map_err(ConfigFileWriteError::Io)?;
        }

        Ok(())
    }
}

/// A path to the actual config file.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum ConfigFilePath {
    /// Default path based on the OS's default configuration directory.
    #[default]
    Default,
    /// Custom configuration directory path.
    Custom(PathBuf),
}

impl ConfigFilePath {
    pub fn as_path(&self) -> &Path {
        match self {
            Self::Default => Path::new(DEFAULT_CONFIG_FILE_PATH.as_str()),
            Self::Custom(pathbuf) => pathbuf.as_path(),
        }
    }
}

/// A configuration file.
///
/// Represents a `config.json` file in a configuration directory. Used purely for serialization and deserialization, so should not be public.
/// [`Sob`] is used here so we can serialize a borrowed [`Config`], while deserializing to an owned [`Config`].
/// [`Cow`] can't be used instead since it requires the inner type to implement [`Clone`].
#[derive(Serialize, Deserialize)]
enum ConfigFile<'a> {
    /// A file containing actual configuration data.
    Config(Sob<'a, Config>),
    /// A redirect to the actual configuration file path.
    Redirect(PathBuf),
}

#[derive(Debug, Error)]
pub enum ConfigFileReadError {
    #[error("config file read IO error: {}", .0)]
    Io(#[from] std::io::Error),
    #[error("config file deserialization error: {}", .0)]
    Deserialize(#[from] serde_json::Error),
    #[error("config file error: too many redirects")]
    TooManyRedirects,
}

#[derive(Debug, Error)]
pub enum ConfigFileWriteError {
    #[error("config file write IO error: {}", .0)]
    Io(#[from] std::io::Error),
    #[error("config file serialization error: {}", .0)]
    Serialize(#[from] serde_json::Error),
}
