/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // sanity check first
    code.chars()
        .rev()
        .try_fold(
            (0, 0, 0, 0),
            |(total_count, count9, sum1, sum2), c| match c {
                '0'..='9' => {
                    let digit = (c as u8) - b'0';
                    Some(match (total_count % 2, digit > 4) {
                        (1, true) => (total_count + 1, count9 + 1, sum1, sum2 + digit),
                        (1, false) => (total_count + 1, count9, sum1, sum2 + digit),
                        _ => (total_count + 1, count9, sum1 + digit, sum2),
                    })
                }
                ' ' => Some((total_count, count9, sum1, sum2)),
                _ => None,
            },
        )
        .map_or(false, |(total_count, count9, sum1, sum2)| {
            total_count > 1 && ((sum1 + sum2 * 2 - 9 * count9) % 10 == 0)
        })
}
