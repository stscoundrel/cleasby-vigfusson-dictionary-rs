mod dictionary_entry;
mod dictionary_location;
use dictionary_location::DictionaryLocation;
pub use dictionary_entry::DictionaryEntry;
use crate::reader;

fn get_dictionary_dataset(dictionary_location: DictionaryLocation) -> Result<Vec<DictionaryEntry>, &'static str> {
    let dictionary_path = dictionary_location.get_path();
    let contents = reader::read_json_file(dictionary_path).unwrap();

    match serde_json::from_str(&contents){
        Ok(entries) => Ok(entries),
        Err(_e) => Err("Failed to serialize dictionary to DictionaryEntries"),
    }
}

/// Get full list of dictionary words.
/// This version contains <strong>strong</strong> and <i>italic</i> HTML tags
/// to match the layout of the printed book.
///
/// 
/// # Examples
/// 
/// ```
/// use cleasby_vigfusson_dictionary::{get_dictionary, DictionaryEntry};
/// 
/// let dictionary: Vec<DictionaryEntry> = get_dictionary().unwrap();
/// 
/// println!("First word is {}, first definition for it being {}", &dictionary[0].word, &dictionary[0].definitions[0])
/// ```
pub fn get_dictionary() -> Result<Vec<DictionaryEntry>, &'static str> {
    get_dictionary_dataset(DictionaryLocation::MarkupDictionary)
}

/// Get full list of dictionary words.
/// This version does not contain additional HTML formatting or any tags.
///
/// 
/// # Examples
/// 
/// ```
/// use cleasby_vigfusson_dictionary::{get_no_markup_dictionary, DictionaryEntry};
/// 
/// let dictionary: Vec<DictionaryEntry> = get_no_markup_dictionary().unwrap();
/// 
/// println!("First word is {}, first definition for it being {}", &dictionary[0].word, &dictionary[0].definitions[0])
/// ```
pub fn get_no_markup_dictionary() -> Result<Vec<DictionaryEntry>, &'static str> {
    get_dictionary_dataset(DictionaryLocation::NoMarkupDictionary)
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