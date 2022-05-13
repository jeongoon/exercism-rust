pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3];

    match n {
        0 => 2,
        1 => 3,
        _ => (5..)
            .step_by(2)
            .filter(|&candi| {
                if !primes.iter().any(|&p| candi % p == 0) {
                    primes.push(candi);
                    true
                } else {
                    false
                }
            })
            .nth((n - 2) as usize)
            .unwrap(),
    }
}

fn main() {
    let n = 4;
    println!("What is the 0-indexed {}th prime number: {} ", n, nth(n));
}
