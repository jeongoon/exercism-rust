#![allow(unused)]
trait DivisibleBy<T = Self> {
    fn divisible_by(self, rhs: T) -> bool;
}

impl DivisibleBy for u64 {
    fn divisible_by(self, rhs: u64) -> bool {
        self % rhs == 0
    }
}

pub fn is_leap_year(year: u64) -> bool {
    year.divisible_by(400) || (!year.divisible_by(100) && year.divisible_by(4))
}

fn main() {
    println!("{:?}", is_leap_year(1996));
}
