use std::string::FromUtf8Error;

use thiserror::Error;

use self::dictionary::{Dictionary, DictionaryResult};

pub mod dictionary;

pub struct Database {
    dictionaries: Vec<Box<dyn Dictionary>>,
}

impl Database {
    fn add_dictionary(&mut self, dictionary: impl Dictionary + 'static) {
        self.dictionaries.push(Box::new(dictionary));
    }

    fn get(&self, key: &str) -> Vec<(&dyn Dictionary, DictionaryResult)> {
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
    use super::*;

    #[test]
    fn basic() {
        // TODO
    }
}
