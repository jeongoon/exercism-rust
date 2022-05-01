pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec!["".to_string(); digits.len() + 1],
        _ => digits
            .chars()
            .collect::<Vec<char>>()
            .windows(len)
            .map(|c| c.iter().collect::<String>())
            .collect(),
    }
}

fn main() {
    println!("{:?}", series("92017", 3));
}
