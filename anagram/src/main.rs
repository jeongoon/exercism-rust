use std::collections::HashSet;

struct AnagramWord {
    uppered: String,
    uppered_sorted: String,
}

impl AnagramWord {
    fn new(w: &str) -> AnagramWord {
        let ws = w.to_uppercase().chars().collect::<Vec<char>>();
        let mut ws_srtd = ws.clone();
        ws_srtd.sort_unstable();

        AnagramWord {
            uppered: ws.iter().collect(),
            uppered_sorted: ws_srtd.iter().collect(),
        }
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let key = AnagramWord::new(word);

    possible_anagrams
        .iter()
        .filter_map(|&w| -> Option<&str> {
            let candi = AnagramWord::new(w);
            if candi.uppered != key.uppered && candi.uppered_sorted == key.uppered_sorted {
                Some(w)
            } else {
                None
            }
        })
        .collect()
}

fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
    let result = anagrams_for(word, inputs);

    let expected: HashSet<&str> = expected.iter().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_no_matches() {
    let word = "diaper";

    let inputs = ["hello", "world", "zombies", "pants"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_detect_simple_anagram() {
    let word = "ant";

    let inputs = ["tan", "stand", "at"];

    let outputs = vec!["tan"];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_does_not_confuse_different_duplicates() {
    let word = "galea";

    let inputs = ["eagle"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_eliminate_anagram_subsets() {
    let word = "good";

    let inputs = ["dog", "goody"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_detect_anagram() {
    let word = "listen";

    let inputs = ["enlists", "google", "inlets", "banana"];

    let outputs = vec!["inlets"];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_multiple_anagrams() {
    let word = "allergy";

    let inputs = [
        "gallery",
        "ballerina",
        "regally",
        "clergy",
        "largely",
        "leading",
    ];

    let outputs = vec!["gallery", "regally", "largely"];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_case_insensitive_anagrams() {
    let word = "Orchestra";

    let inputs = ["cashregister", "Carthorse", "radishes"];

    let outputs = vec!["Carthorse"];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_unicode_anagrams() {
    let word = "??????";

    // These words don't make sense, they're just greek letters cobbled together.

    let inputs = ["??????", "??????", "??????"];

    let outputs = vec!["??????", "??????"];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_misleading_unicode_anagrams() {
    // Despite what a human might think these words contain different letters, the input uses Greek

    // A and B while the list of potential anagrams uses Latin A and B.

    let word = "??????";

    let inputs = ["AB??"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_does_not_detect_a_word_as_its_own_anagram() {
    let word = "banana";

    let inputs = ["banana"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_does_not_detect_a_differently_cased_word_as_its_own_anagram() {
    let word = "banana";

    let inputs = ["bAnana"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_does_not_detect_a_differently_cased_unicode_word_as_its_own_anagram() {
    let word = "??????";

    let inputs = ["??????"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_same_bytes_different_chars() {
    let word = "a???"; // 61 E2 AC 82

    let inputs = ["???a"]; // E2 82 AC 61

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

#[test]
fn test_different_words_but_same_ascii_sum() {
    let word = "bc";

    let inputs = ["ad"];

    let outputs = vec![];

    process_anagram_case(word, &inputs, &outputs);
}

// no main
// test with:
// shell> cargo test
fn main() {}
