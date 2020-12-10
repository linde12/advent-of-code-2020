use std::collections::{HashMap, HashSet};

fn count_required_bags(rules: &HashMap<String, HashSet<(usize, String)>>, name: String) -> usize {
    let mut sum = 0;
    if let Some(colors) = rules.get(name.as_str()) {
        for (number, color) in colors {
            sum += number + number * count_required_bags(&rules, color.to_owned());
        }
    }
    sum
}

pub fn solve(input: &str) -> usize {
    let rules = crate::parse_rules(input);
    count_required_bags(&rules, "shiny gold".into())
}

#[test]
fn test_2() {
    let input = include_str!("./input_test2.txt").trim();
    let result = solve(&input);
    assert_eq!(result, 126)
}
