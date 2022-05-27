pub fn is_pangram(sentence: &str) -> bool {
    let collect_as_uppercases = |mut acc: String, c: char| {
        if c.is_ascii_alphabetic() {
            let uc = c.to_ascii_uppercase();
            if !acc.contains(uc) {
                acc.push(uc)
            }
        }
        acc
    };

    sentence
        .chars()
        .fold(String::new(), collect_as_uppercases)
        .len()
        == 26
}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog";

    assert!(is_pangram(sentence));
}
