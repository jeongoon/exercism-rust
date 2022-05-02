use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<String>()
}

fn main() {
    println!("{}", reverse("test  asjdska"));
}
