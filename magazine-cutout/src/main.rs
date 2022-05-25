#![allow(unused)]
use std::collections::{hash_map::Entry::Occupied, hash_map::Entry::Vacant, HashMap};
use std::iter::{Flatten, Map};

pub fn can_construct_note<'a>(magazine: &'a [&str], note: &'a [&str]) -> bool {
    let split_no_alphabets = |splited_by_spaced: &'a [&str]| {
        splited_by_spaced
            .iter()
            .map(|&m| m.split(|c| !char::is_alphabetic(c)))
            .flatten()
    };

    let mut mag = HashMap::new();

    split_no_alphabets(magazine).for_each(|w| match (mag.entry(w)) {
        Occupied(o) => *o.into_mut() += 1,
        Vacant(v) => {
            v.insert(1);
        }
    });

    for n in split_no_alphabets(note) {
        match mag.get_mut(n) {
            None | Some(0) => {
                return false;
            }
            Some(x) => {
                *x -= 1;
            }
        }
    }

    true
}

pub fn main() {
    let magazine = "Astronomer Amy Mainzer spent hours chatting with Leonardo DiCaprio for Netflix's 'Don't Look Up'".split_whitespace().collect::<Vec<&str>>();
    let note = "Amy Mainzer chatting with Leonardo DiCaprio."
        .split_whitespace()
        .collect::<Vec<&str>>();
    assert!(can_construct_note(&magazine, &note));
}
