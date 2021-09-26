use cleasby_vigfusson_dictionary::{get_dictionary, get_no_markup_dictionary, DictionaryEntry};
use insta::assert_json_snapshot;

#[test]
fn gets_default_dictionary() {    
    let result = get_dictionary().unwrap();

    assert_json_snapshot!(result)
}

#[test]
fn gets_no_markup_dictionary() {    
    let result = get_no_markup_dictionary().unwrap();

    assert_json_snapshot!(result)
}

#[test]
fn exports_dictionary_entry() {    
    let entry = DictionaryEntry {
        word: String::from("Lorem ipsum"),
        definitions: vec!(
            String::from("Dolor sit amet"),
            String::from("Dolor sit igitur"),
        )
    };

    assert_eq!(entry.word, "Lorem ipsum");
    assert_eq!(entry.definitions[0], "Dolor sit amet");
    assert_eq!(entry.definitions[1], "Dolor sit igitur");
}