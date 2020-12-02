use std::collections::HashSet;

fn main() {
    let input = include_str!("./1.txt").trim();
    let numbers: HashSet<i32> = input.lines().map(|s| s.parse().expect("input is not all numbers")).collect();

    // part 1
    if let Some(result) = solve_1(&numbers) {
        println!("part 1 solution: {}", result);
    }

    // part 2
    if let Some(result) = solve_2(&numbers) {
        println!("part 2 solution: {}", result);
    }
}

fn solve_1(input: &HashSet<i32>) -> Option<i32> {
    input.iter().find_map(|num| {
        if input.contains(&(2020-num)) {
            Some(num * (2020-num))
        } else {
            None
        }
    })
}

fn solve_2(input: &HashSet<i32>) -> Option<i32> {
    for num in input.iter() {
        for num2 in input.iter() {
            if input.contains(&(2020-num-num2)) {
                return Some(num * num2 * (2020-num-num2));
            }
        }
    }
    None
}

#[test]
fn test_1() {
    let input: HashSet<i32> = vec![2000, 20, 123, 4].into_iter().collect();
    let result = solve_1(&input);
    assert_eq!(result, Some(40000));
}

#[test]
fn test_2() {
    let input = vec![83, 2000, 19, 1].into_iter().collect();
    let result = solve_2(&input);
    assert_eq!(result, Some(38000));
}
