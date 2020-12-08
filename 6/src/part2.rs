use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let groups: Vec<&str> = input.split("\n\n").collect();

    let group_answers: Vec<usize> = groups
        .iter()
        .map(|group| {
            let mut answers: HashMap<char, usize> = HashMap::new();
            let n_people_in_group = group.split_whitespace().count();

            // Set how many people in the group answered "yes" to the same questions
            group
                .chars()
                .skip_while(|c| c.is_whitespace())
                .for_each(|ch| {
                    answers.entry(ch).and_modify(|e| *e += 1).or_insert(1);
                });

            // Count the amount of questions everyone in the group answered "yes" to
            let c = answers
                .iter()
                .filter(|(_ch, n_ans)| **n_ans == n_people_in_group)
                .count();
            c
        })
        .collect();

    // For each group, get the sum of all the questions everyone answered "yes" to
    group_answers.iter().sum()
}

#[test]
fn test_2() {
    let input = include_str!("./input_test.txt").trim();
    let result = solve(&input);
    assert_eq!(result, 6)
}
