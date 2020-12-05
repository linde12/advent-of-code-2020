use std::collections::{HashMap, VecDeque};

fn bsp_parse(bp: &mut VecDeque<char>, (lower, upper): (usize, usize)) -> usize {
    if let Some(ch) = bp.pop_front() {
        let next_range = match ch {
            'F' | 'L' => (lower, (upper + lower) / 2),
            _ => ((upper + lower) / 2, upper),
        };
        bsp_parse(bp, next_range)
    } else {
        lower
    }
}

pub fn solve(input: &str) -> Option<usize> {
    let boardingpasses: Vec<_> = input.lines().collect();

    boardingpasses
        .iter()
        .map(|bp| {
            let (row, col) = bp.split_at(7);
            let row_no = bsp_parse(&mut row.chars().collect(), (0, 128));
            let col_no = bsp_parse(&mut col.chars().collect(), (0, 8));
            row_no * 8 + col_no
        })
        .max()
}

#[test]
fn test_1() {
    let input = include_str!("./input_test.txt").trim();
    let result = solve(&input);
    assert_eq!(result, Some(820))
}
