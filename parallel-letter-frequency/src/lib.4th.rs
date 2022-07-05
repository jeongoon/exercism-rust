use std::collections::{hash_map::Entry::Occupied, hash_map::Entry::Vacant, HashMap};

use std::sync::mpsc;

use std::thread;

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let num_workers = if worker_count == 0 { 1 } else { worker_count };

    // make a channel for all the results collected
    let (tx_main, rx_main) = mpsc::channel();

    let tx_threads = (0..num_workers)
        .map(|_id| {
            // preprare for sending
            let tx = tx_main.clone();
            // prepare for listening
            let (tx_to_thread, rx_from_main) = mpsc::channel::<Option<&'static str>>();

            thread::spawn(move || {
                // collect count
                let mut subtotal = HashMap::new();

                loop {
                    if let Some(s) = rx_from_main.recv().unwrap() {
                        // got string -> count them
                        s.chars().for_each(|lc| {
                            if lc.is_alphabetic() {
                                lc.to_lowercase().for_each(|ch| match subtotal.entry(ch) {
                                    Occupied(o) => *o.into_mut() += 1,
                                    Vacant(v) => {
                                        v.insert(1);
                                    }
                                });
                            };
                        });
                    } else {
                        // return the value and finish the thread
                        tx.send(subtotal).unwrap();
                        break;
                    }
                }
            });

            tx_to_thread
        })
        .collect::<Vec<_>>();

    let mut total = HashMap::<char, usize>::new();

    input
        .iter()
        .zip(tx_threads.iter().cycle())
        .for_each(|(s, tx)| {
            tx.send(Some(s)).unwrap();
        });

    tx_threads.iter().for_each(|tx| {
        tx.send(None).unwrap();
        let subtotal = rx_main.recv().unwrap();

        subtotal
            .iter()
            .for_each(|(&ch, &count)| match total.entry(ch) {
                Occupied(o) => *o.into_mut() += count,

                Vacant(v) => {
                    v.insert(count);
                }
            });
    });

    total
}
