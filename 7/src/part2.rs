use std::collections::HashMap;

pub fn count_required_bags(rules: &HashMap<String, Vec<(usize, String)>>, name: String) -> usize {
    let mut sum = 0;
    if let Some(colors) = rules.get(name.as_str()) {
        for (number, color) in colors {
            sum += number + number * count_required_bags(&rules, color.to_owned());
        }
    }
    sum
}

pub fn solve(input: &str) -> usize {
    let raw_rules: Vec<&str> = input.lines().collect();
    let mut rules: HashMap<String, Vec<(usize, String)>> = HashMap::new();

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
                    let number: String = part.chars().take_while(|c| !c.is_whitespace()).collect();
                    let number = number.parse::<usize>().unwrap();
                    let contained_bag: Vec<&str> = part.split(" ").skip(1).take(2).collect();
                    let contained_bag = contained_bag.join(" ");
                    Some((number, contained_bag))
                }
            })
            .for_each(|pair| {
                let entry = rules.entry(container.to_owned()).or_insert(vec![]);
                entry.push(pair);
            });
    });

    count_required_bags(&rules, "shiny gold".into())
}

#[test]
fn test_2() {
    let input = include_str!("./input_test2.txt").trim();
    let result = solve(&input);
    assert_eq!(result, 126)
}
