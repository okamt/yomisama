use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use serde::{
    de::{self, DeserializeSeed, Error, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::database::dictionary::{importer, DictionaryBuilder, DictionaryEntry};

use super::Importer;

pub struct JMDictSimplifiedImporter {}

impl Importer for JMDictSimplifiedImporter {
    fn import<DB>(
        path: impl AsRef<Path>,
        dict_builder: DB,
    ) -> Result<DB::Dictionary, importer::Error>
    where
        DB: DictionaryBuilder,
    {
        let reader = BufReader::new(File::open(path)?);
        let mut deserializer = serde_json::Deserializer::from_reader(reader);
        let jmdict_deserializer = JMDictDeserializer { dict_builder };
        let jmdict = jmdict_deserializer.deserialize(&mut deserializer)?;

        Ok(jmdict.dict_builder.build()?)
    }
}

struct JMDict<DB>
where
    DB: DictionaryBuilder,
{
    common_only: bool,
    dict_date: String,
    dict_revisions: Vec<String>,
    languages: Vec<String>,
    tags: HashMap<String, String>,
    version: String,
    dict_builder: DB,
}

struct JMDictDeserializer<DB>
where
    DB: DictionaryBuilder,
{
    dict_builder: DB,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct JMDictWord {
    #[serde(skip)]
    #[allow(dead_code)]
    id: String,
    kana: Vec<JMDictKana>,
    kanji: Vec<JMDictKanji>,
    sense: Vec<JMDictSense>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct JMDictKana {
    applies_to_kanji: Vec<String>,
    #[serde(skip)]
    #[allow(dead_code)]
    common: bool,
    #[serde(skip)]
    #[allow(dead_code)]
    tags: Vec<String>,
    text: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct JMDictKanji {
    #[serde(skip)]
    #[allow(dead_code)]
    common: bool,
    #[serde(skip)]
    #[allow(dead_code)]
    tags: Vec<String>,
    text: String,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct JMDictSense {
    #[serde(skip)]
    #[allow(dead_code)]
    antonym: (),
    applies_to_kana: Vec<String>,
    applies_to_kanji: Vec<String>,
    dialect: Vec<String>,
    field: Vec<String>,
    gloss: Vec<JMDictGloss>,
    info: Vec<String>,
    #[serde(skip)]
    #[allow(dead_code)]
    language_source: (),
    misc: Vec<String>,
    part_of_speech: Vec<String>,
    #[serde(skip)]
    #[allow(dead_code)]
    related: (),
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct JMDictGloss {
    #[serde(skip)]
    #[allow(dead_code)]
    gender: (),
    #[serde(skip)]
    #[allow(dead_code)]
    lang: String,
    text: String,
    #[serde(rename(deserialize = "type"))]
    #[serde(skip)]
    #[allow(dead_code)]
    type_: Option<String>,
}

impl<'de, DB> DeserializeSeed<'de> for JMDictDeserializer<DB>
where
    DB: DictionaryBuilder + 'de,
{
    type Value = JMDict<DB>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JMDictVisitor<DB: DictionaryBuilder> {
            dict_builder: DB,
        }

        struct JMDictWordsSeed<DB: DictionaryBuilder> {
            dict_builder: DB,
        }

        impl<'de, DB> DeserializeSeed<'de> for JMDictWordsSeed<DB>
        where
            DB: DictionaryBuilder,
        {
            type Value = DB;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct JMDictWordsVisitor<DB: DictionaryBuilder>(JMDictWordsSeed<DB>);

                impl<'de, DB> Visitor<'de> for JMDictWordsVisitor<DB>
                where
                    DB: DictionaryBuilder,
                {
                    type Value = DB;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("array of JMDictWord")
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: SeqAccess<'de>,
                    {
                        let mut dict_builder = self.0.dict_builder;

                        while let Some(word) = seq.next_element::<JMDictWord>()? {
                            for sense in word.sense {
                                let mut tags = Vec::with_capacity(
                                    sense.dialect.len()
                                        + sense.field.len()
                                        + sense.misc.len()
                                        + sense.part_of_speech.len(),
                                );
                                tags.extend(sense.dialect);
                                tags.extend(sense.field);
                                tags.extend(sense.misc);
                                tags.extend(sense.part_of_speech);

                                let applies_to_kanji = match sense.applies_to_kanji.first() {
                                    Some(s) if s == "*" => {
                                        word.kanji.iter().map(|k| k.text.clone()).collect()
                                    }
                                    _ => sense.applies_to_kanji,
                                };

                                let applies_to_kana = match sense.applies_to_kana.first() {
                                    Some(s) if s == "*" => {
                                        word.kana.iter().map(|k| k.text.clone()).collect()
                                    }
                                    _ => sense.applies_to_kana,
                                };

                                let gloss = sense
                                    .gloss
                                    .iter()
                                    .map(|gloss| gloss.text.as_ref())
                                    .collect::<Vec<&str>>()
                                    .join("\n");

                                for applies_to in applies_to_kana.iter() {
                                    let entry = DictionaryEntry {
                                        readings: vec![applies_to.clone()],
                                        gloss: gloss.clone(),
                                        tags: tags.clone(),
                                    };

                                    dict_builder
                                        .add(applies_to, entry)
                                        .map_err(A::Error::custom)?;
                                }

                                for applies_to in applies_to_kanji.iter() {
                                    let readings = word
                                        .kana
                                        .iter()
                                        .filter_map(|kana| {
                                            if kana.applies_to_kanji.first().map(|s| s as &str)
                                                == Some("*")
                                                || kana.applies_to_kanji.contains(applies_to)
                                            {
                                                Some(kana.text.clone())
                                            } else {
                                                None
                                            }
                                        })
                                        .collect();

                                    let entry = DictionaryEntry {
                                        readings,
                                        gloss: gloss.clone(),
                                        tags: tags.clone(),
                                    };

                                    dict_builder
                                        .add(applies_to, entry)
                                        .map_err(A::Error::custom)?;
                                }
                            }
                        }

                        Ok(dict_builder)
                    }
                }

                deserializer.deserialize_seq(JMDictWordsVisitor(self))
            }
        }

        impl<'de, DB> Visitor<'de> for JMDictVisitor<DB>
        where
            DB: DictionaryBuilder + 'de,
        {
            type Value = JMDict<DB>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("JMDict struct")
            }

            fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut common_only = Default::default();
                let mut dict_date = Default::default();
                let mut dict_revisions = Default::default();
                let mut languages = Default::default();
                let mut tags = Default::default();
                let mut version = Default::default();

                const FIELDS: &[&str] = &[
                    "commonOnly",
                    "dictDate",
                    "dictRevisions",
                    "languages",
                    "tags",
                    "version",
                    "words",
                ];

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_ref() {
                        "commonOnly" => {
                            common_only = map.next_value()?;
                        }
                        "dictDate" => {
                            dict_date = map.next_value()?;
                        }
                        "dictRevisions" => {
                            dict_revisions = map.next_value()?;
                        }
                        "languages" => {
                            languages = map.next_value()?;
                        }
                        "tags" => {
                            tags = map.next_value()?;
                        }
                        "version" => {
                            version = map.next_value()?;
                        }
                        "words" => {
                            self.dict_builder = map.next_value_seed(JMDictWordsSeed {
                                dict_builder: self.dict_builder,
                            })?;
                        }
                        unknown => {
                            return Err(de::Error::unknown_field(unknown, FIELDS));
                        }
                    }
                }

                Ok(JMDict {
                    common_only,
                    dict_date,
                    dict_revisions,
                    languages,
                    tags,
                    version,
                    dict_builder: self.dict_builder,
                })
            }
        }

        deserializer.deserialize_map(JMDictVisitor {
            dict_builder: self.dict_builder,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::database::dictionary::{cdb::CDBDictionaryBuilder, Dictionary};

    use super::*;

    #[test]
    fn basic() {
        let temp_dir = tempfile::tempdir().expect("could not create temp dir");

        let text = r#"{
"version": "3.5.0",
"languages": ["eng"],
"commonOnly": false,
"dictDate": "2023-12-04",
"dictRevisions": ["1.09","1.08","1.07","1.06","1.05","1.04"],
"tags": {"v5uru":"Godan verb - Uru old class verb (old form of Eru)","v2g-s":"Nidan verb (lower class) with 'gu' ending (archaic)","dei":"deity"},
"words": [
{"id":"1000320","kanji":[{"common":false,"text":"彼処","tags":["rK"]},{"common":false,"text":"彼所","tags":["rK"]}],"kana":[{"common":true,"text":"あそこ","tags":[],"appliesToKanji":["*"]},{"common":false,"text":"あすこ","tags":[],"appliesToKanji":["*"]},{"common":false,"text":"かしこ","tags":[],"appliesToKanji":["*"]},{"common":false,"text":"アソコ","tags":[],"appliesToKanji":[]},{"common":false,"text":"あしこ","tags":["ok"],"appliesToKanji":["*"]},{"common":false,"text":"あこ","tags":["ok"],"appliesToKanji":["*"]}],"sense":[{"partOfSpeech":["pn"],"appliesToKanji":["*"],"appliesToKana":["*"],"related":[["どこ",1],["ここ",1],["そこ",1]],"antonym":[],"field":[],"dialect":[],"misc":["uk"],"info":["place physically distant from both speaker and listener"],"languageSource":[],"gloss":[{"lang":"eng","gender":null,"type":null,"text":"there"},{"lang":"eng","gender":null,"type":null,"text":"over there"},{"lang":"eng","gender":null,"type":null,"text":"that place"},{"lang":"eng","gender":null,"type":null,"text":"yonder"},{"lang":"eng","gender":null,"type":null,"text":"you-know-where"}]},{"partOfSpeech":["n"],"appliesToKanji":["*"],"appliesToKana":["あそこ","あすこ","アソコ"],"related":[],"antonym":[],"field":[],"dialect":[],"misc":["col","uk","euph"],"info":[],"languageSource":[],"gloss":[{"lang":"eng","gender":null,"type":null,"text":"genitals"},{"lang":"eng","gender":null,"type":null,"text":"private parts"},{"lang":"eng","gender":null,"type":null,"text":"nether regions"}]},{"partOfSpeech":["n"],"appliesToKanji":["*"],"appliesToKana":["*"],"related":[["あれほど"]],"antonym":[],"field":[],"dialect":[],"misc":["uk"],"info":["something psychologically distant from both speaker and listener"],"languageSource":[],"gloss":[{"lang":"eng","gender":null,"type":null,"text":"that far"},{"lang":"eng","gender":null,"type":null,"text":"that much"},{"lang":"eng","gender":null,"type":null,"text":"that point"}]}]},
{"id":"1000360","kanji":[],"kana":[{"common":true,"text":"あっさり","tags":[],"appliesToKanji":["*"]},{"common":false,"text":"アッサリ","tags":[],"appliesToKanji":["*"]}],"sense":[{"partOfSpeech":["adv","adv-to","vs"],"appliesToKanji":["*"],"appliesToKana":["*"],"related":[],"antonym":[],"field":[],"dialect":[],"misc":["on-mim"],"info":[],"languageSource":[],"gloss":[{"lang":"eng","gender":null,"type":null,"text":"easily"},{"lang":"eng","gender":null,"type":null,"text":"readily"},{"lang":"eng","gender":null,"type":null,"text":"quickly"},{"lang":"eng","gender":null,"type":null,"text":"flatly (refuse)"}]},{"partOfSpeech":["adv","adv-to","vs"],"appliesToKanji":["*"],"appliesToKana":["*"],"related":[],"antonym":[],"field":[],"dialect":[],"misc":["on-mim"],"info":[],"languageSource":[],"gloss":[{"lang":"eng","gender":null,"type":null,"text":"lightly (seasoned food, applied make-up, etc.)"},{"lang":"eng","gender":null,"type":null,"text":"plainly"},{"lang":"eng","gender":null,"type":null,"text":"simply"}]}]},
{"id":"1000390","kanji":[{"common":true,"text":"あっという間に","tags":[]},{"common":false,"text":"あっと言う間に","tags":[]},{"common":false,"text":"アッという間に","tags":["sK"]},{"common":false,"text":"アッと言う間に","tags":["sK"]},{"common":false,"text":"あっとゆう間に","tags":["sK"]},{"common":false,"text":"アッとゆう間に","tags":["sK"]}],"kana":[{"common":true,"text":"あっというまに","tags":[],"appliesToKanji":["*"]}],"sense":[{"partOfSpeech":["exp","adv"],"appliesToKanji":["*"],"appliesToKana":["*"],"related":[],"antonym":[],"field":[],"dialect":[],"misc":[],"info":[],"languageSource":[],"gloss":[{"lang":"eng","gender":null,"type":null,"text":"in an instant"},{"lang":"eng","gender":null,"type":null,"text":"in a flash"},{"lang":"eng","gender":null,"type":null,"text":"in the blink of an eye"},{"lang":"eng","gender":null,"type":null,"text":"in no time at all"},{"lang":"eng","gender":null,"type":null,"text":"just like that"}]}]}
]}
"#;
        let dict_path = temp_dir
            .path()
            .with_file_name("jmdict-simplified")
            .with_extension("json");
        fs::write(&dict_path, text).expect("could not write dictionary file to temp dir");

        let path = temp_dir
            .path()
            .with_file_name("jmdict-simplified-test-basic");
        let dict_builder =
            CDBDictionaryBuilder::new(path.to_str().expect("cdb database path is not valid utf-8"))
                .unwrap();
        let jmdict = JMDictSimplifiedImporter::import(&dict_path, dict_builder)
            .expect("error while importing dictionary file");

        let expected = r#"Ok([DictionaryEntry { readings: ["あそこ", "あすこ", "かしこ", "あしこ", "あこ"], gloss: "there\nover there\nthat place\nyonder\nyou-know-where", tags: ["uk", "pn"] }, DictionaryEntry { readings: ["あそこ", "あすこ", "かしこ", "あしこ", "あこ"], gloss: "genitals\nprivate parts\nnether regions", tags: ["col", "uk", "euph", "n"] }, DictionaryEntry { readings: ["あそこ", "あすこ", "かしこ", "あしこ", "あこ"], gloss: "that far\nthat much\nthat point", tags: ["uk", "n"] }])"#;
        assert_eq!(format!("{:?}", jmdict.get("彼処")), expected);
    }
}
