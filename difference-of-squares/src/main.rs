pub fn sum(n: u32) -> u32 {
    n * (n + 1) / 2
}

pub fn square_of_sum(n: u32) -> u32 {
    let s = sum(n);
    s * s
}

pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference1(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

pub fn difference2(n: u32) -> u32 {
    n * (n + 1) / 2 * n * (n + 1) / 2 - n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference3(n: u32) -> u32 {
    n * (n + 1) * (n * (n + 1) / 4 - (2 * n + 1) / 6)
}

pub fn difference4(n: u32) -> u32 {
    n * (n + 1) * (3 * n * (n + 1) / 12 - (4 * n + 2) / 12)
}

pub fn difference5(n: u32) -> u32 {
    (n * (n + 1)) * (3 * n * n - n - 2) / 12
}

pub fn difference(n: u32) -> u32 {
    let sqn = n * n;
    (sqn + n) * (3 * sqn - n - 2) / 12
}

fn main() {
    println!(
        "difference between square_of_sum({0}) and sum_of_squares({0}) is {1}",
        10,
        difference4(10)
    );
}
