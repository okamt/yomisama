//! Database querying module.

use api::database::dictionary::DictionaryEntry;
use serde::{Deserialize, Serialize};

/// An entry for a lookup query, to be sent to the frontend.
///
/// Compared to [`DictionaryEntry`], note the added `word` field, since the entry key might not be the exact same as the original lookup input.
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryEntry {
    pub word: String,
    pub word_alt: Vec<String>,
    pub text: String,
    pub tags: Vec<String>,
}

impl QueryEntry {
    /// Converts a [`DictionaryEntry`] to a [`QueryEntry`].
    pub fn from_dictionary_entry(entry: DictionaryEntry, word: String) -> Self {
        Self {
            word,
            word_alt: entry.readings,
            text: entry.gloss,
            tags: entry.tags,
        }
    }
}
