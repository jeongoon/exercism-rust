pub fn nth(mut n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3];
    let mut candi = 3;

    match n {
        0 => 2,
        1 => 3,
        _ => {
            'new_number: while n > 1 {
                candi += 2;
                n -= 1;
                for p in primes.iter() {
                    if candi % p == 0 {
                        continue 'new_number;
                    }
                }
                n -= 1;
                primes.push(candi);
            }
            primes.last().unwrap()
        }
    }
}

fn main() {
    let n = 4;
    println!("What is the 0-indexed {}th prime number: {} ", n, nth(n));
}
