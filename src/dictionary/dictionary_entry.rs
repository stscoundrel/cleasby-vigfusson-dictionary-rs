use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub word: String,
    pub definitions: Vec<String>
}