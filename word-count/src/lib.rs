use std::collections::{hash_map::Entry::Occupied, hash_map::Entry::Vacant, HashMap};

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut wc: HashMap<String, u32> = HashMap::new();
    let is_separator = |c| -> bool { c != '\'' && !char::is_alphanumeric(c) };
    let sanitize_word = |str: &str| -> String { str.trim_matches('\'').to_lowercase() };

    words
        .split(is_separator)
        .filter(|w| !str::is_empty(w))
        .for_each(|word| {
            let sanitized_word = sanitize_word(word);
            println!("{}", sanitized_word);

            match wc.entry(sanitized_word) {
                Occupied(o) => *o.into_mut() += 1,
                Vacant(v) => {
                    v.insert(1);
                }
            }
        });
    wc
}
