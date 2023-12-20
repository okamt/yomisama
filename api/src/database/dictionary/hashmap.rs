use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{Dictionary, DictionaryBuilder, DictionaryEntry, DictionaryMetadata};

pub struct HashMapDictionaryBuilder {
    hashmap: HashMap<String, DictionaryEntry>,
}

impl HashMapDictionaryBuilder {
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
        }
    }
}

impl DictionaryBuilder for HashMapDictionaryBuilder {
    type Dictionary = HashMapDictionary;
    type Error = Error;

    fn add(&mut self, key: &str, entry: DictionaryEntry) -> Result<(), Self::Error> {
        self.hashmap.insert(key.to_owned(), entry);
        Ok(())
    }

    fn build(self, metadata: DictionaryMetadata) -> Result<Self::Dictionary, Self::Error> {
        Ok(HashMapDictionary {
            hashmap: self.hashmap,
            metadata,
        })
    }
}

#[derive(Debug, Error)]
pub enum Error {}

#[derive(Serialize, Deserialize, Clone)]
pub struct HashMapDictionary {
    hashmap: HashMap<String, DictionaryEntry>,
    metadata: DictionaryMetadata,
}

impl Dictionary for HashMapDictionary {
    fn get(&self, key: &str) -> Vec<DictionaryEntry> {
        self.hashmap
            .get(key)
            .map(Clone::clone)
            .into_iter()
            .collect()
    }

    fn get_metadata(&self) -> &DictionaryMetadata {
        &self.metadata
    }
}
