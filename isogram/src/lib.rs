trait ToAsciiAlphaBit {
    fn to_ascii_alpha_bit(self) -> u32;
}

impl ToAsciiAlphaBit for char {
    fn to_ascii_alpha_bit(self) -> u32 {
        let lc = self.to_ascii_lowercase();
        1u32 << ((lc as u8) - b'a')
    }
}

pub fn check(candidate: &str) -> bool {
    candidate
        .chars()
        //.filter_map(|c| c.is_alphabetic().then(|| c.to_ascii_alpha_bit()))
        .try_fold(0u32, |bits, c| {
            if c.is_alphabetic() {
                let b = c.to_ascii_alpha_bit();
                ((bits & b) == 0).then(|| bits | b)
            } else {
                Some(bits)
            }
        })
        .is_some()
}
