use serde::{Deserialize, Serialize};

use self::dictionary::{Dictionary, DictionaryEntry};

pub mod dictionary;

#[derive(Serialize, Deserialize)]
pub struct Database<D: Dictionary> {
    pub dictionaries: Vec<D>,
}

impl<D: Dictionary> Database<D> {
    pub fn new() -> Self {
        Self {
            dictionaries: Vec::new(),
        }
    }

    pub fn add_dictionary(&mut self, dictionary: D) {
        self.dictionaries.push(dictionary);
    }

    pub fn get(&self, key: &str) -> Vec<(&D, Vec<DictionaryEntry>)> {
        self.dictionaries.iter().map(|d| (d, d.get(key))).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::database::dictionary::hashmap::HashMapDictionary;

    use super::{
        dictionary::{hashmap::HashMapDictionaryBuilder, DictionaryBuilder, DictionaryEntry},
        *,
    };

    #[test]
    fn basic() {
        let mut database = Database::new();
        let mut dict_builder = HashMapDictionaryBuilder::new();
        let dict_entry = DictionaryEntry {
            readings: vec!["あける".to_owned()],
            gloss: "to open (a door, etc.), to unwrap (e.g. parcel, package), to unlock".to_owned(),
            tags: vec!["P".to_owned(), "v1".to_owned(), "vt".to_owned()],
        };
        dict_builder.add("test", dict_entry.clone()).unwrap();
        let dict = dict_builder.build(Default::default()).unwrap();
        database.add_dictionary(dict.clone());

        let serialized = serde_json::to_string(&database).expect("could not serialize database");
        let deserialized = serde_json::from_str::<Database<HashMapDictionary>>(&serialized)
            .expect("could not deserialize database");

        assert_eq!(
            deserialized.get("test").first().unwrap().1.first().unwrap(),
            &dict_entry
        );
    }
}
