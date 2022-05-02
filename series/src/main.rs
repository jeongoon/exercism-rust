pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        vec![]
    } else {
        let max_idx = digits.len() - len + 1;

        (0..max_idx)
            .map(|i| digits[i..i + len].to_string())
            .collect()
    }
}

fn main() {
    println!("{:?}", series("92017", 3));
}
