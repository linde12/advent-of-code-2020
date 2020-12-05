mod part1;
mod part2;

fn main() {
    let input = include_str!("./input.txt").trim();

    // part 1
    let result = part1::solve(input);
    println!("part 1 solution: {}", result);

    // part 2
    let result = part2::solve(input);
    println!("part 2 solution: {}", result);
}
