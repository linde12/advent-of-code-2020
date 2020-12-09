use std::collections::{HashMap, HashSet};

pub fn find_outer_bags(
    rules: &HashMap<String, Vec<&str>>,
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

pub fn solve(input: &str) -> usize {
    let raw_rules: Vec<&str> = input.lines().collect();
    let mut rules: HashMap<String, Vec<&str>> = HashMap::new();

    raw_rules.iter().for_each(|raw| {
        let parts: Vec<&str> = raw.split(" bags contain ").collect();
        let container = parts[0]; // e.g. "shiny gold"
        let raw = parts[1];

        let parts: Vec<&str> = raw.split(", ").collect();
        parts
            .iter()
            .filter_map(|part| {
                if *part == "no other bags." {
                    None
                } else {
                    let contained_bag: Vec<&str> = part.split(" ").skip(1).take(2).collect();
                    let contained_bag = contained_bag.join(" ");
                    Some(contained_bag)
                }
            })
            .for_each(|contained| { // e.g. "dark red"
                rules
                    .entry(contained)
                    .and_modify(|c| c.push(container))
                    .or_insert(vec![container]);
            });
    });

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
