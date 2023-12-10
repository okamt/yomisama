use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Dictionary, DictionaryBuilder, DictionaryEntry, DictionaryMetadata, DictionaryResult};

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

    fn add(&mut self, key: &str, entry: DictionaryEntry) -> Result<(), super::Error> {
        self.hashmap.insert(key.to_owned(), entry);
        Ok(())
    }

    fn build(self, metadata: DictionaryMetadata) -> Result<Self::Dictionary, super::Error> {
        Ok(HashMapDictionary {
            hashmap: self.hashmap,
            metadata,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HashMapDictionary {
    hashmap: HashMap<String, DictionaryEntry>,
    metadata: DictionaryMetadata,
}

#[typetag::serde]
impl Dictionary for HashMapDictionary {
    fn get(&self, key: &str) -> DictionaryResult {
        Ok(self
            .hashmap
            .get(key)
            .map(Clone::clone)
            .into_iter()
            .collect())
    }

    fn get_metadata(&self) -> &DictionaryMetadata {
        &self.metadata
    }
}
