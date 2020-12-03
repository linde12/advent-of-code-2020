use std::collections::HashSet;

// After making the initial solution and seeing that both tasks
// only cared about the trees (and not the free spaces) i've
// refactored to use a HashSet.
// I've also made several other improvements:
// - Removed unnecessary `step` closure, now i instead pass the
// increment values
// - Use `.chars()` to iterate over chars... D'oh.
// - Use `.product()` over `fold` to get the product for solve_2
// - Get rid of `Terrain` enum. Not needed when for the HashSet
// implementation.

fn main() {
    let input = include_str!("./input.txt").trim();
    let (map, width, height) = parse_map(input);

    // part 1
    let result = solve_1(&map, width, height, 3, 1);
    println!("part 1 solution: {}", result);

    // part 2
    let result = solve_2(&map, width, height);
    println!("part 2 solution: {}", result);
}

fn parse_map(map_str: &str) -> (HashSet<(usize, usize)>, usize, usize) {
    let map: HashSet<(usize, usize)> = map_str
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, ch)| match ch {
                    '#' => Some((x, y)),
                    _ => None,
                })
                .collect::<HashSet<(usize, usize)>>()
        })
        .collect();

    let height = map_str.lines().count();
    let width = map_str
        .lines()
        .nth(0)
        .map(|line| line.len())
        .expect("Unable to get map width");

    (map, width, height)
}

fn solve_1(
    map: &HashSet<(usize, usize)>,
    map_width: usize,
    map_height: usize,
    inc_x: usize,
    inc_y: usize,
) -> usize {
    let mut trees_encountered = 0;
    let (mut x, mut y) = (inc_x, inc_y);

    while y < map_height {
        // Check what we landed on
        let wrapped_x = x % map_width;
        if map.contains(&(wrapped_x, y)) {
            trees_encountered += 1;
        }

        // Move
        x += inc_x;
        y += inc_y;
    }

    trees_encountered
}

fn solve_2(map: &HashSet<(usize, usize)>, map_width: usize, map_height: usize) -> usize {
    vec![
        solve_1(map, map_width, map_height, 1, 1),
        solve_1(map, map_width, map_height, 3, 1),
        solve_1(map, map_width, map_height, 5, 1),
        solve_1(map, map_width, map_height, 7, 1),
        solve_1(map, map_width, map_height, 1, 2),
    ]
    .iter()
    .product()
}

#[test]
fn test_parse_map() {
    let input = include_str!("./input_test.txt");
    let (map, _, _) = parse_map(input);
    assert_eq!(map.contains(&(3, 1)), false);
    assert_eq!(map.contains(&(4, 1)), true);
    assert_eq!(map.contains(&(5, 1)), false);
}

#[test]
fn test_1() {
    let input = include_str!("./input_test.txt");
    let (map, width, height) = parse_map(input);
    let result = solve_1(&map, width, height, 3, 1);
    assert_eq!(result, 7);
    let result = solve_1(&map, width, height, 1, 1);
    assert_eq!(result, 2);
}

#[test]
fn test_2() {
    let input = include_str!("./input_test.txt");
    let (map, width, height) = parse_map(input);
    let result = solve_2(&map, width, height);
    assert_eq!(result, 336);
}
