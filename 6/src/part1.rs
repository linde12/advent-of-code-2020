use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let groups: Vec<&str> = input.split("\n\n").collect();
    let group_answers: Vec<HashSet<char>> = groups
        .iter()
        .map(|group| {
            group
                .chars()
                .filter_map(|ch| match ch {
                    '\n' => None,
                    c => Some(c),
                })
                .collect()
        })
        .collect();

    group_answers.iter().map(|answers| answers.len()).sum()
}

#[test]
fn test_1() {
    let input = include_str!("./input_test.txt").trim();
    let result = solve(&input);
    assert_eq!(result, 11)
}
