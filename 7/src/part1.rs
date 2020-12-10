use std::collections::{HashMap, HashSet};

fn find_outer_bags(
    rules: &HashMap<String, HashSet<String>>,
    mut visited: &mut HashSet<String>,
    name: String,
) {
    if let Some(colors) = rules.get(name.as_str()) {
        for color in colors {
            if !visited.contains(&color.to_string()) {
                visited.insert(color.to_string());
                find_outer_bags(&rules, &mut visited, color.to_string());
            }
        }
    }
}

fn inverted_no_num(
    rules: &HashMap<String, HashSet<(usize, String)>>,
) -> HashMap<String, HashSet<String>> {
    rules
        .iter()
        .fold(HashMap::new(), |mut acc, (name, contains)| {
            for (_, c) in contains {
                let entry = acc.entry(c.clone()).or_insert(HashSet::new());
                entry.insert(name.clone());
            }
            acc
        })
}

pub fn solve(input: &str) -> usize {
    let rules = crate::parse_rules(input);
    let rules = inverted_no_num(&rules);

    let mut visited: HashSet<String> = HashSet::new();
    find_outer_bags(&rules, &mut visited, "shiny gold".into());
    visited.len()
}

#[test]
fn test_1() {
    let input = include_str!("./input_test.txt").trim();
    let result = solve(&input);
    assert_eq!(result, 4)
}
