use std::path::PathBuf;

use cdb::{CDBWriter, CDB};

use super::{Dictionary, DictionaryBuilder, DictionaryEntry};

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

    fn add(&mut self, key: &str, entry: DictionaryEntry) -> Result<(), super::Error> {
        self.cdb_writer
            .add(key.as_bytes(), &entry.serialize()?)
            .map_err(super::Error::Io)
    }

    fn build(self) -> Result<Self::Dictionary, super::Error> {
        self.cdb_writer.finish().map_err(super::Error::Io)?;
        Ok(Self::Dictionary {
            cdb: CDB::open(self.path)?,
        })
    }
}

pub struct CDBDictionary {
    cdb: CDB,
}

impl Dictionary for CDBDictionary {
    fn get(&self, key: &str) -> Result<Vec<DictionaryEntry>, super::Error> {
        self.cdb
            .find(key.as_bytes())
            .map(|r| {
                r.map_err(super::Error::Io)
                    .and_then(|ref v| DictionaryEntry::deserialize(v))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(unused_variables)]
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

        let cdb_dict = cdb_dict_builder.build().unwrap();
        assert_eq!(*cdb_dict.get("test1").unwrap().first().unwrap(), test1);
        assert_eq!(*cdb_dict.get("test2").unwrap().first().unwrap(), test2);
        assert!(cdb_dict.get("test3").unwrap().is_empty());
    }
}
