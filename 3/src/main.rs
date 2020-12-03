use std::collections::HashSet;

// Yet another solution. Possibly the simplest, yet
// least robust. Using iterators and `nth` to index
// into the string without converting to HashSet or
// Vec.

fn main() {
    let input = include_str!("./input.txt").trim();

    // part 1
    if let Some(result) = solve_1(&input, 3, 1) {
        println!("part 1 solution: {}", result);
    } else {
        println!("unable to find solution to part 1, check input");
    }

    // part 2
    let result = solve_2(&input);
    println!("part 2 solution: {}", result);
}

fn solve_1(map: &str, inc_x: usize, inc_y: usize) -> Option<usize> {
    let height = map.lines().count();
    // assume all lines have the same length
    let width = map.lines().nth(0).map(|line| line.len())?;

    let mut trees_encountered = 0;
    let (mut x, mut y) = (inc_x, inc_y);

    while y < height {
        let line = map.lines().nth(y)?;
        if line.chars().nth(x % width)? == '#' {
            trees_encountered += 1;
        }

        // Move
        x += inc_x;
        y += inc_y;
    }

    Some(trees_encountered)
}

fn solve_2(map: &str) -> usize {
    vec![
        solve_1(map, 1, 1),
        solve_1(map, 3, 1),
        solve_1(map, 5, 1),
        solve_1(map, 7, 1),
        solve_1(map, 1, 2),
    ]
    .into_iter()
    .filter_map(|x| x)
    .product()
}

#[test]
fn test_1() {
    let input = include_str!("./input_test.txt");
    let result = solve_1(&input, 3, 1);
    assert_eq!(result, Some(7));
    let result = solve_1(&input, 1, 1);
    assert_eq!(result, Some(2));
}

#[test]
fn test_2() {
    let input = include_str!("./input_test.txt");
    let result = solve_2(&input);
    assert_eq!(result, 336);
}
