use semver::Version;
use serde::{Deserialize, Serialize};
use url::Url;

pub mod cdb;
pub mod hashmap;
pub mod importer;

pub trait DictionaryBuilder {
    type Dictionary: Dictionary;
    type Error: std::error::Error;

    fn add(&mut self, key: &str, entry: DictionaryEntry) -> Result<(), Self::Error>;
    fn build(self, metadata: DictionaryMetadata) -> Result<Self::Dictionary, Self::Error>;
}

pub trait Dictionary {
    fn get(&self, key: &str) -> Vec<DictionaryEntry>;
    fn get_metadata(&self) -> &DictionaryMetadata;
}

#[derive(Debug, bitcode::Encode, bitcode::Decode, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct DictionaryEntry {
    pub readings: Vec<String>,
    pub gloss: String,
    pub tags: Vec<String>,
}

impl DictionaryEntry {
    pub fn serialize_fast(&self) -> Vec<u8> {
        bitcode::encode(self).unwrap()
    }

    pub fn deserialize_fast(data: &[u8]) -> Self {
        bitcode::decode(data).unwrap()
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

impl Default for DictionaryMetadata {
    fn default() -> Self {
        Self {
            name: Default::default(),
            author: Default::default(),
            version: Version::new(0, 0, 0),
            homepage_url: Default::default(),
            update_url: Default::default(),
            notes: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dict_entry_fast_serialization() {
        let dict_entry = DictionaryEntry {
            readings: vec!["あける".to_owned()],
            gloss: "to open (a door, etc.), to unwrap (e.g. parcel, package), to unlock".to_owned(),
            tags: vec!["P".to_owned(), "v1".to_owned(), "vt".to_owned()],
        };

        let serialized = dict_entry.serialize_fast();
        let deserialized = DictionaryEntry::deserialize_fast(&serialized);

        assert_eq!(dict_entry, deserialized);
    }
}
