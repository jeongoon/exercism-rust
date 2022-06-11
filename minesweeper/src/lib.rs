pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let nine_cells = |x: usize, max_count: usize| {
        // note: this capture actually includes the center position as well
        // but `find_mines` will only exectued on non-mine cell
        // so the cell istelf always doesn't have a mine.
        (x.saturating_sub(1))..(x + 1).min(max_count.saturating_sub(1)).saturating_add(1)
    };

    let find_mines = |r: usize, c: usize| -> usize {
        nine_cells(r, minefield.len())
            .map(|r1| {
                minefield
                    .get(r1 as usize)
                    .map(|&row| {
                        //println!("{} for {} {}", r1, r, row);
                        row.get(nine_cells(c, row.len()))
                            .map(|ns| ns.chars().filter(|c| c == &'*').count())
                            .unwrap_or(0)
                    })
                    .unwrap_or(0)
            })
            .sum::<usize>()
    };

    minefield
        .iter()
        .enumerate()
        .map(|(row, &rowstr)| {
            rowstr
                .chars()
                .enumerate()
                .map(|(col, ch)| match ch {
                    '*' => '*',
                    _ => match find_mines(row, col) {
                        0 => ' ',
                        k => char::from_digit(k as u32, 10).unwrap(),
                    },
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}
