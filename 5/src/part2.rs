use std::collections::{HashMap, HashSet, VecDeque};

fn bsp_parse(bp: &mut VecDeque<char>, (lower, upper): (usize, usize)) -> usize {
    if let Some(ch) = bp.pop_front() {
        let next_range = match ch {
            'F' | 'L' => (lower, (upper + lower) / 2),
            _ => (
                (upper + lower) / 2 + if (upper + lower) % 2 == 1 { 1 } else { 0 },
                upper,
            ),
        };
        bsp_parse(bp, next_range)
    } else {
        lower
    }
}

pub fn solve(input: &str) -> Option<usize> {
    let boardingpasses: Vec<_> = input.lines().collect();

    let seats: HashSet<usize> = boardingpasses
        .iter()
        .filter_map(|bp| {
            let (row, col) = bp.split_at(7);
            let row_no = bsp_parse(&mut row.chars().collect(), (0, 127));
            let col_no = bsp_parse(&mut col.chars().collect(), (0, 7));

            if row_no != 0 && row_no != 127 {
                Some(row_no * 8 + col_no)
            } else {
                None
            }
        })
        .collect();

    // this can most likely be improved, but cba...
    seats.iter().find_map(|seat| {
        if !seats.contains(&(*seat - 1)) && seats.contains(&(*seat - 2)) {
            Some(*seat - 1)
        } else if !seats.contains(&(*seat + 1)) && seats.contains(&(*seat + 2)) {
            Some(*seat + 1)
        } else {
            None
        }
    })
}
