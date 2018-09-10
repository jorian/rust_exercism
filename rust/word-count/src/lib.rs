use std::collections::HashMap;

/// Count occurrences of words.

pub fn word_count(words: &str) -> HashMap<String, u32> {

    let mut hm = HashMap::new();

    words
        .split_whitespace()
        .map(| word | word.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|c| !c.is_empty())
        .map(| word | word.to_ascii_lowercase())
        .for_each(| word | {
            let counter = hm.entry(word.to_string()).or_insert(0);
            *counter += 1;
        });

    hm
}
