#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = dividend / divisor;
    let remainder = dividend - quotient * divisor;

    (quotient, remainder)
}

pub fn evens_<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate()
        .filter(|(i, val)| i % 2 == 0)
        .map(|(_, val)| val)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate().filter_map(|(i, val)| -> Option<T> {
        if i % 2 == 0 {
            Some(val)
        } else {
            None
        }
    })
}

pub struct Position(pub i16, pub i16);

impl Position {
    // https://en.wikipedia.org/wiki/Taxicab_geometry
    // origin : (0,0)
    pub fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}

fn main() {
    assert_eq!(divmod(10, 3), (3, 1));
    assert_eq!(divmod(10, -3), (-3, 1));
    let mut even_ints = evens(0_u8..);
    assert_eq!(even_ints.next(), Some(0));
    assert_eq!(even_ints.next(), Some(2));
    assert_eq!(even_ints.next(), Some(4));
    assert_eq!(even_ints.next(), Some(6));
    assert_eq!(Position(3, 4).manhattan(), 7);
}
