const MAX_SQUARES: u32 = 64;

pub fn square(s: u32) -> u64 {
    if s == 0 || s > MAX_SQUARES {
        panic!("Square must be between 1 and 64")
    } else {
        2_u64.pow(s - 1)
    }
}

pub fn total_bruteforce() -> u64 {
    (1..=MAX_SQUARES).map(square).sum::<u64>()
}

pub fn total() -> u64 {
    u64::max_value()
}
