use std::str::Utf8Error;

use semver::Version;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

pub mod cdb;
pub mod importer;

pub trait DictionaryBuilder {
    type Dictionary: Dictionary;

    fn add(&mut self, key: &str, entry: DictionaryEntry) -> Result<(), Error>;
    fn build(self, metadata: DictionaryMetadata) -> Result<Self::Dictionary, Error>;
}

pub type DictionaryResult = Result<Vec<DictionaryEntry>, Error>;

#[typetag::serde(tag = "type")]
pub trait Dictionary {
    fn get(&self, key: &str) -> DictionaryResult;
    fn get_metadata(&self) -> &DictionaryMetadata;
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("dictionary IO error")]
    Io(#[from] std::io::Error),
    #[error("dictionary UTF-8 error")]
    Utf8(#[from] Utf8Error),
    #[error("dictionary serialization error")]
    Serialization(#[from] bitcode::Error),
}

#[derive(Debug, bitcode::Encode, bitcode::Decode, PartialEq, Eq, Clone)]
pub struct DictionaryEntry {
    pub readings: Vec<String>,
    pub gloss: String,
    pub tags: Vec<String>,
}

impl DictionaryEntry {
    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        bitcode::encode(self).map_err(Error::Serialization)
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, Error> {
        bitcode::decode(data).map_err(Error::Serialization)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct DictionaryMetadata {
    name: String,
    author: String,
    version: Version,
    homepage_url: Option<Url>,
    update_url: Option<Url>,
    notes: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dict_entry_serialization() {
        let dict_entry = DictionaryEntry {
            readings: vec!["あける".to_owned()],
            gloss: "to open (a door, etc.), to unwrap (e.g. parcel, package), to unlock".to_owned(),
            tags: vec!["P".to_owned(), "v1".to_owned(), "vt".to_owned()],
        };

        let serialized = dict_entry.serialize().expect("serialization error");
        let deserialized = DictionaryEntry::deserialize(&serialized)
            .expect("serialized data should not have invalid utf-8");

        assert_eq!(dict_entry, deserialized);
    }
}
