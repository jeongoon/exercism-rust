use std::collections::{hash_map::Entry::Occupied, hash_map::Entry::Vacant, HashMap};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord(usize, usize);

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut acc = HashMap::<Coord, usize>::new();
    let row_size = minefield.len();
    let col_sizes = minefield.iter().map(|&s| s.len()).collect::<Vec<usize>>();

    let (mines, mine_frees, mut annotated) = minefield.iter().enumerate().fold(
        (
            Vec::<Coord>::new(),
            Vec::<Coord>::new(),
            Vec::<String>::new(),
        ),
        |(mut mines, mut mine_frees, mut annotated), (r, &minerow)| {
            minerow.chars().enumerate().for_each(|(c, ch)| {
                let mut annorow_string = String::new();

                match ch {
                    '*' => {
                        mines.push(Coord(r, c));
                        annorow_string.push('*');
                    }
                    _ => {
                        mine_frees.push(Coord(r, c));
                        annorow_string.push(' ');
                    }
                }
                annotated.push(annorow_string);
            });
            (mines, mine_frees, annotated)
        },
    );

    mines
        .iter()
        .map(|coord| {
            (((coord.1 as i64) - 1)..((coord.1 as i64) + 1))
                .filter_map(move |y| {
                    if 0 <= y && y <= (row_size as i64) {
                        Some(
                            (((coord.0 as i64) - 1)..((coord.0 as i64) + 1))
                                .filter_map(move |x| {
                                    if 0 <= x
                                        && &(x as usize) <= col_sizes.get(y as usize).unwrap()
                                        && !((coord.0 as i64) == x && (coord.1 as i64) == x)
                                    {
                                        Some(x)
                                    } else {
                                        None
                                    }
                                })
                                .map(move |x_| Coord(x_ as usize, y as usize)),
                        )
                    } else {
                        None
                    }
                })
                .flatten()
        })
        .flatten()
        .for_each(|coord| match acc.entry(coord) {
            Occupied(o) => *o.into_mut() += 1,
            Vacant(v) => {
                v.insert(1);
            }
        });

    mine_frees.iter().for_each(|coord| {
        annotated.get_mut(coord.1).map(|row_str| {
            row_str
                .get_mut(coord.0..coord.0)
                .map(|_| acc.entry(*coord).or_default().to_string())
        });
    });

    annotated
}
