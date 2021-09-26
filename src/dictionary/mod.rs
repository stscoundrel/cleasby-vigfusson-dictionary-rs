mod dictionary_entry;
use dictionary_entry::DictionaryEntry;
use crate::reader;
use serde_json;

const DICTIONARY_PATH: &str = "src/dictionary/dataset/cleasby-vigfusson.json";
const NO_MARKUP_DICTIONARY_PATH: &str = "src/dictionary/dataset/cleasby-vigfusson-no-markup.json";

fn get_dictionary_dataset(dictionary_path: String) -> Result<Vec<DictionaryEntry>, &'static str> {
    let contents = reader::read_json_file(dictionary_path).unwrap();

    match serde_json::from_str(&contents){
        Ok(entries) => Ok(entries),
        Err(_e) => Err("Failed to serialize dictionary to DictionaryEntries"),
    }
}

pub fn get_dictionary() -> Result<Vec<DictionaryEntry>, &'static str> {
    get_dictionary_dataset(String::from(DICTIONARY_PATH))
}

pub fn get_no_markup_dictionary() -> Result<Vec<DictionaryEntry>, &'static str> {
    get_dictionary_dataset(String::from(NO_MARKUP_DICTIONARY_PATH))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_expected_entry_content() {
        let result = get_dictionary();
        let dictionary = result.unwrap();
        let entry: &DictionaryEntry = &dictionary[1989];

        assert_eq!(entry.word, "át-frekr");
        assert_eq!(entry.definitions[0], "adj. <i>greedy, voracious,</i> Hkv. 2. 41.");
    }

    #[test]
    fn dictionary_has_35207_entries() {
        let result = get_dictionary();
        let dictionary = result.unwrap();

        assert_eq!(dictionary.len(), 35207);
    }

    #[test]
    fn no_markup_dictionary_has_expected_entry_content() {
        let result = get_no_markup_dictionary();
        let dictionary = result.unwrap();
        let entry: &DictionaryEntry = &dictionary[1989];

        assert_eq!(entry.word, "át-frekr");
        assert_eq!(entry.definitions[0], "adj. greedy, voracious, Hkv. 2. 41.");
    }

    #[test]
    fn no_markup_dictionary_has_35207_entries() {
        let result = get_no_markup_dictionary();
        let dictionary = result.unwrap();

        assert_eq!(dictionary.len(), 35207);
    }
}