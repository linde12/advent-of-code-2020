#[derive(Debug, PartialEq)]
enum Terrain {
    Tree,
    Space,
}

fn main() {
    let input = include_str!("./input.txt").trim();
    let map = parse_map(input);

    // part 1
    let result = solve_1(&map, |x, y| (x + 3, y + 1));
    println!("part 1 solution: {}", result);

    // part 2
    let result = solve_2(&map);
    println!("part 2 solution: {}", result);
}

fn parse_map(map_str: &str) -> Vec<Vec<Terrain>> {
    let map: Vec<Vec<Terrain>> = map_str
        .lines()
        .map(|line| {
            let parts: Vec<Terrain> = line
                .split("")
                .filter_map(|rune| {
                    if let Some(ch) = rune.chars().nth(0) {
                        match ch {
                            '#' => Some(Terrain::Tree),
                            '.' => Some(Terrain::Space),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .collect();
            parts
        })
        .collect();
    map
}

fn map_get(map: &Vec<Vec<Terrain>>, x: usize, y: usize) -> Option<&Terrain> {
    map.get(y)?.get(x)
}

fn solve_1<F>(map: &Vec<Vec<Terrain>>, step: F) -> usize
where
    F: Fn(usize, usize) -> (usize, usize),
{
    let mut trees_encountered = 0;
    let (mut x, mut y) = step(0, 0);

    while y < map.len() {
        // Check what we landed on
        let wrapped_x = x % map[y].len();
        if let Some(Terrain::Tree) = map_get(map, wrapped_x, y) {
            trees_encountered += 1;
        }

        // Move
        let (new_x, new_y) = step(x, y);
        x = new_x;
        y = new_y;
    }

    trees_encountered
}

fn solve_2(map: &Vec<Vec<Terrain>>) -> usize {
    vec![
        solve_1(map, |x, y| (x + 1, y + 1)),
        solve_1(map, |x, y| (x + 3, y + 1)),
        solve_1(map, |x, y| (x + 5, y + 1)),
        solve_1(map, |x, y| (x + 7, y + 1)),
        solve_1(map, |x, y| (x + 1, y + 2)),
    ]
    .iter()
    .fold(1, |prev, next| prev * next)
}

#[test]
fn test_parse_map() {
    let input = include_str!("./input_test.txt");
    let map = parse_map(input);
    assert_eq!(map[1][3], Terrain::Space);
    assert_eq!(map[1][4], Terrain::Tree);
    assert_eq!(map[1][5], Terrain::Space);
}

#[test]
fn test_1() {
    let input = include_str!("./input_test.txt");
    let map = parse_map(input);
    let result = solve_1(&map, |x, y| (x + 3, y + 1));
    assert_eq!(result, 7);
    let result = solve_1(&map, |x, y| (x + 1, y + 1));
    assert_eq!(result, 2);
}

#[test]
fn test_2() {
    let input = include_str!("./input_test.txt");
    let map = parse_map(input);
    let result = solve_2(&map);
    assert_eq!(result, 336);
}
