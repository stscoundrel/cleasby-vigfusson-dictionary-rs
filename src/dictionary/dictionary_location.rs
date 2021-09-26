const DICTIONARY_PATH: &str = "src/dictionary/dataset/cleasby-vigfusson.json";
const NO_MARKUP_DICTIONARY_PATH: &str = "src/dictionary/dataset/cleasby-vigfusson-no-markup.json";

pub enum DictionaryLocation {
    MarkupDictionary,
    NoMarkupDictionary,
}

impl DictionaryLocation { 
    pub fn get_path(&self) -> String { 
        match *self { 
            DictionaryLocation::MarkupDictionary => String::from(DICTIONARY_PATH), 
            DictionaryLocation::NoMarkupDictionary => String::from(NO_MARKUP_DICTIONARY_PATH)
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_dictionary_paths() {
        let result1 = DictionaryLocation::MarkupDictionary.get_path();
        let result2 = DictionaryLocation::NoMarkupDictionary.get_path();

        assert_eq!(result1, DICTIONARY_PATH);
        assert_eq!(result2, NO_MARKUP_DICTIONARY_PATH);
    }
}