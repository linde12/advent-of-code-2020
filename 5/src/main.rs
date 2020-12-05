mod part1;
mod part2;

fn main() {
    let input = include_str!("./input.txt").trim();

    // part 1
    if let Some(result) = part1::solve(input) {
        println!("part 1 solution: {}", result);
    } else {
        println!("bad input for part 1");
    }

    // part 2
    if let Some(result) = part2::solve(input) {
        println!("part 1 solution: {}", result);
    } else {
        println!("bad input for part 2");
    }
}
