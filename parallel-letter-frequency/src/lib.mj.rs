use std::collections::{hash_map::Entry::Occupied, hash_map::Entry::Vacant, HashMap};

use std::sync::mpsc;

use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let num_workers = if worker_count == 0 { 1 } else { worker_count };

    // make a channel for all the results collected

    let (tx, rx) = mpsc::channel();

    let mut handles = vec![];

    for k in 0..num_workers {
        // preprare for sending

        let tx1 = tx.clone();

        // prepare for input

        let input_for_worker = input
            .iter()
            .flat_map(|&str| {
                let chars = str.chars();

                let chars_count = chars.clone().count();

                let slice_size_ = chars_count / num_workers;

                let slice_remainder = chars_count % num_workers;

                let (skip_size, take_size) = match &k {
                    &0 => (0, slice_size_ + slice_remainder),

                    _ => (k * slice_size_ + slice_remainder, slice_size_),
                };

                chars.skip(skip_size).take(take_size)
            })
            .collect::<Vec<_>>();

        let handle = thread::spawn(move || {
            // collect count

            let mut hm = HashMap::<char, usize>::new();

            input_for_worker.iter().for_each(|c| {
                let lc = c.to_ascii_lowercase();

                if lc.is_alphabetic() {
                    match hm.entry(lc) {
                        Occupied(o) => *o.into_mut() += 1,

                        Vacant(v) => {
                            v.insert(1);
                        }
                    }
                }
            });

            // send it to the mainstream

            tx1.send(hm).unwrap();
        });

        handles.push(handle);
    }

    let mut total_hm = HashMap::<char, usize>::new();

    for handle in handles {
        let received = rx.recv().unwrap();

        received
            .iter()
            .for_each(|(&ch, &count)| match total_hm.entry(ch) {
                Occupied(o) => *o.into_mut() += count,

                Vacant(v) => {
                    v.insert(count);
                }
            });

        //handle.join().expect("the cihld thread panicked!");
    }

    total_hm
}
