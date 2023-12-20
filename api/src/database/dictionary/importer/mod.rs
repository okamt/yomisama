use std::path::Path;

use thiserror::Error;

use super::DictionaryBuilder;

pub mod jmdict_simplified;

pub trait Importer: Sized {
    type Error: std::error::Error;

    fn import<DB>(
        path: impl AsRef<Path>,
        dict_builder: DB,
    ) -> Result<DB::Dictionary, Error<Self::Error, DB::Error>>
    where
        DB: DictionaryBuilder;
}

#[derive(Debug, Error)]
pub enum Error<IE: std::error::Error, DBE: std::error::Error> {
    #[error("dictionary file IO error")]
    DictFileIo(std::io::Error),
    #[error(transparent)]
    DictBuilder(DBE),
    #[error(transparent)]
    ImporterSpecific(IE),
}
