use std::collections::{hash_map::Entry::Occupied, hash_map::Entry::Vacant, HashMap};

use std::sync::mpsc;

use std::thread;

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let num_workers = if worker_count == 0 { 1 } else { worker_count };
    let mut handles = vec![];

    // make a channel for all the results collected
    let (tx_main, rx_main) = mpsc::channel();

    let mut input_per_worker = vec![vec![]; num_workers];
    for (&s, i) in input.iter().zip((0..worker_count).cycle()) {
        input_per_worker.get_mut(i).unwrap().push(s);
    }

    for input_ in input_per_worker {
        // preprare for sending
        let tx = tx_main.clone();
        // prepare for listening
        let handle = thread::spawn(move || {
            // collect count
            let mut subtotal = HashMap::<char, usize>::new();

            for &s in input_.iter() {
                for ch in s.chars() {
                    if ch.is_alphabetic() {
                        ch.to_lowercase().for_each(|lc| match subtotal.entry(lc) {
                            Occupied(o) => *o.into_mut() += 1,
                            Vacant(v) => {
                                v.insert(1);
                            }
                        });
                    };
                }
            }

            tx.send(subtotal).unwrap();
        });
        handles.push(handle);
    }

    handles.iter().fold(HashMap::new(), |mut acc, _handle| {
        for (&ch, &count) in rx_main.recv().unwrap().iter() {
            match acc.entry(ch) {
                Occupied(o) => *o.into_mut() += count,
                Vacant(v) => {
                    v.insert(count);
                }
            }
        }
        acc
    })
}
