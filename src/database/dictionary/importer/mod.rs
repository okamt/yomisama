use std::path::Path;

use thiserror::Error;

use super::DictionaryBuilder;
use crate::database::dictionary;

pub mod jmdict_simplified;

pub trait Importer {
    fn import<DB>(path: impl AsRef<Path>, dict_builder: DB) -> Result<DB::Dictionary, Error>
    where
        DB: DictionaryBuilder;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("dictionary importing io error")]
    Io(#[from] std::io::Error),
    #[error("dictionary JSON parsing error")]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Dictionary(#[from] dictionary::Error),
}
