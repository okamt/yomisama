use std::string::FromUtf8Error;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use self::dictionary::{Dictionary, DictionaryResult};

pub mod dictionary;

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub dictionaries: Vec<Box<dyn Dictionary>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            dictionaries: Vec::new(),
        }
    }

    pub fn add_dictionary(&mut self, dictionary: impl Dictionary + 'static) {
        self.dictionaries.push(Box::new(dictionary));
    }

    pub fn get(&self, key: &str) -> Vec<(&dyn Dictionary, DictionaryResult)> {
        self.dictionaries
            .iter()
            .map(|d| (d.as_ref(), d.get(key)))
            .collect()
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("utf-8 error")]
    Utf8(#[from] std::str::Utf8Error),
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        value.utf8_error().into()
    }
}

#[cfg(test)]
mod tests {
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
        let deserialized =
            serde_json::from_str::<Database>(&serialized).expect("could not deserialize database");

        assert_eq!(
            deserialized
                .get("test")
                .first()
                .unwrap()
                .1
                .as_ref()
                .unwrap()
                .first()
                .unwrap(),
            &dict_entry
        );
    }
}
