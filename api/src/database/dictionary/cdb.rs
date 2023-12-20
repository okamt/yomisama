use std::path::PathBuf;

use cdb::{CDBWriter, CDB};
use serde::{
    de::{self, Visitor},
    ser, Deserialize, Deserializer, Serialize, Serializer,
};
use thiserror::Error;

use super::{Dictionary, DictionaryBuilder, DictionaryEntry, DictionaryMetadata};

pub struct CDBDictionaryBuilder {
    cdb_writer: CDBWriter,
    path: PathBuf,
}

impl CDBDictionaryBuilder {
    pub fn new<P: Into<PathBuf> + ToString>(path: P) -> Result<Self, std::io::Error> {
        Ok(Self {
            cdb_writer: CDBWriter::create(path.to_string())?,
            path: path.into(),
        })
    }
}

impl DictionaryBuilder for CDBDictionaryBuilder {
    type Dictionary = CDBDictionary;
    type Error = CDBDictionaryBuilderError;

    fn add(&mut self, key: &str, entry: DictionaryEntry) -> Result<(), Self::Error> {
        self.cdb_writer
            .add(key.as_bytes(), &entry.serialize_fast())
            .map_err(Self::Error::CDBFileIo)
    }

    fn build(self, metadata: DictionaryMetadata) -> Result<Self::Dictionary, Self::Error> {
        self.cdb_writer.finish()?;
        Ok(Self::Dictionary {
            cdb_pathbuf: (CDB::open(&self.path)?, self.path),
            metadata,
        })
    }
}

#[derive(Debug, Error)]
pub enum CDBDictionaryBuilderError {
    #[error("CDB file io error")]
    CDBFileIo(#[from] std::io::Error),
}

#[derive(Serialize, Deserialize)]
pub struct CDBDictionary {
    #[serde(serialize_with = "serialize_cdb", deserialize_with = "deserialize_cdb")]
    cdb_pathbuf: (CDB, PathBuf),
    metadata: DictionaryMetadata,
}

fn serialize_cdb<S>(cdb_pathbuf: &(CDB, PathBuf), serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(
        cdb_pathbuf
            .1
            .to_str()
            .ok_or(ser::Error::custom("path contains invalid UTF-8 characters"))?,
    )
}

fn deserialize_cdb<'de, D>(deserializer: D) -> Result<(CDB, PathBuf), D::Error>
where
    D: Deserializer<'de>,
{
    struct CDBPathBufVisitor;

    impl<'de> Visitor<'de> for CDBPathBufVisitor {
        type Value = (CDB, PathBuf);

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a path to a CDB")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let pathbuf = PathBuf::from(s);
            let cdb = CDB::open(&pathbuf).map_err(de::Error::custom)?;

            Ok((cdb, pathbuf))
        }
    }

    deserializer.deserialize_str(CDBPathBufVisitor)
}

impl Dictionary for CDBDictionary {
    fn get(&self, key: &str) -> Vec<DictionaryEntry> {
        self.cdb_pathbuf
            .0
            .find(key.as_bytes())
            .filter_map(Result::ok)
            .map(|v| DictionaryEntry::deserialize_fast(&v))
            .collect()
    }

    fn get_metadata(&self) -> &DictionaryMetadata {
        &self.metadata
    }
}

#[cfg(test)]
mod tests {
    use semver::Version;
    use url::Url;

    use super::*;

    #[test]
    fn basic() {
        let temp_dir = tempfile::tempdir().expect("could not create temp dir");

        let mut cdb_dict_builder = CDBDictionaryBuilder::new(
            temp_dir
                .path()
                .with_file_name("cdb-test-basic")
                .to_str()
                .expect("cdb database path is not valid utf-8"),
        )
        .unwrap();

        let test1 = DictionaryEntry {
            readings: vec!["abc".to_owned()],
            gloss: "defg".to_owned(),
            tags: vec!["hi".to_owned(), "jk".to_owned()],
        };

        let test2 = DictionaryEntry {
            readings: vec!["lmn".to_owned()],
            gloss: "opqr".to_owned(),
            tags: vec!["st".to_owned(), "uv".to_owned()],
        };

        cdb_dict_builder.add("test1", test1.clone()).unwrap();
        cdb_dict_builder.add("test2", test2.clone()).unwrap();

        let metadata = DictionaryMetadata {
            name: "cdb test basic".to_owned(),
            author: "tomokao".to_owned(),
            version: Version::new(1, 2, 3),
            homepage_url: Some(Url::parse("https://github.com/tomokao/yomisama").unwrap()),
            update_url: None,
            notes: "this is a test dictionary".to_owned(),
        };
        let cdb_dict = cdb_dict_builder.build(metadata.clone()).unwrap();

        assert_eq!(*cdb_dict.get("test1").first().unwrap(), test1);
        assert_eq!(*cdb_dict.get("test2").first().unwrap(), test2);
        assert!(cdb_dict.get("test3").is_empty());
        assert_eq!(cdb_dict.get_metadata(), &metadata);
    }
}
