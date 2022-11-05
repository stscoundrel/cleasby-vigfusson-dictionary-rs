pub enum DictionaryLocation {
    MarkupDictionary,
    NoMarkupDictionary,
}

impl DictionaryLocation {
    pub fn get_dictionary_json(&self) -> &'static str {
        match *self {
            DictionaryLocation::MarkupDictionary => include_str!("./dataset/cleasby-vigfusson.json"),
            DictionaryLocation::NoMarkupDictionary => include_str!("./dataset/cleasby-vigfusson-no-markup.json")
        }
    }
}