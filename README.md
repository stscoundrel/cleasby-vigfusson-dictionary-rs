# Cleasby & Vigfusson Dictionary

The Cleasby &amp; Vigfusson Old Norse to English Dictionary for Rust. The dictionary consists of 35 000+ Old Norse words with English translations.

Based on the classic dictionary by Richard Cleasby and Gudbrand Vigfusson. Also available for [Node.js](https://github.com/stscoundrel/cleasby-vigfusson-dictionary)

### Usage

Dictionary comes in two variants: one with html markup, and one without.

```rust
// Ships two variants, plus DictionaryEntry.
use cleasby_vigfusson_dictionary::{get_dictionary, get_no_markup_dictionary, DictionaryEntry};

// Standard dictionary. Contains "strong" and "i" tags to match the printed book.
let dictionary = get_dictionary();

// No-markup version. Contains no tags or additional markup.
let no_markup_dictionary = get_no_markup_dictionary();

// Both methods return Result, which should always be safe to unwrap.
// Up to you if you wish to just unwrap, or use other error handling method.
let dictionary_content: Vec<DictionaryEntry> = dictionary.unwrap();
let no_markup_dictionary_content: Vec<DictionaryEntry> = no_markup_dictionary.unwrap();

println!("A word from dictionary: {}. First definition for it is: {}", &dictionary_content[0].word, &dictionary_content[0].definitions[0])
```


### Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
cleasby-vigfusson-dictionary = "1.0.0"
```

### About Cleasby & Vigfusson Dictionary

"Icelandic-English" dictionary was started by Richard Cleasby and finished by Gudbrand Vigfusson. It was published in 1874, which leads to there being many public domain versions of the book available.

