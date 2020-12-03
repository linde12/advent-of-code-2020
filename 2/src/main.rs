fn main() {
    let input = include_str!("./input.txt").trim();
    let inputs: Vec<(&str, &str)> = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            match parts[..] {
                [rule, pass] => Some((rule, pass)),
                _ => None,
            }
        })
        .collect();

    // part 1
    let result = solve_1(&inputs);
    println!("part 1 solution: {}", result);

    // part 2
    let result = solve_2(&inputs);
    println!("part 2 solution: {}", result);
}

fn passes_rule_1(rule: &str, pass: &str) -> bool {
    let parts: Vec<&str> = rule.split(' ').collect();
    if let [range, ch] = parts[..] {
        let range: Vec<usize> = range
            .split('-')
            .map(|numstr| numstr.parse().expect("invalid number in rule range"))
            .collect();

        if let [min, max] = range[..] {
            let rule_char = ch.chars().nth(0).expect("no character in rule");
            let num_ch_in_pass = pass.matches(rule_char).count();
            if num_ch_in_pass >= min && num_ch_in_pass <= max {
                return true;
            } else {
                return false;
            }
        }
    }
    false
}

fn passes_rule_2(rule: &str, pass: &str) -> bool {
    let parts: Vec<&str> = rule.split(' ').collect();
    if let [range, ch] = parts[..] {
        let range: Vec<usize> = range
            .split('-')
            .map(|numstr| numstr.parse().expect("invalid number in rule range"))
            .collect();

        if let [first, second] = range[..] {
            let rule_char = ch.chars().nth(0).expect("no character in rule");
            let first_char = pass
                .chars()
                .nth(first - 1)
                .filter(|ch| *ch == rule_char)
                .is_some();

            let second_char = pass
                .chars()
                .nth(second - 1)
                .filter(|ch| *ch == rule_char)
                .is_some();

            return first_char != second_char;
        }
    }
    false
}

fn solve_1(input: &Vec<(&str, &str)>) -> usize {
    input
        .iter()
        .filter_map(|(rule, pass)| {
            if passes_rule_1(rule, pass) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

fn solve_2(input: &Vec<(&str, &str)>) -> usize {
    input
        .iter()
        .filter_map(|(rule, pass)| {
            if passes_rule_2(rule, pass) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

#[test]
fn test_passes_rule_2() {
    assert_eq!(passes_rule_2("1-3 a", "abcde"), true);
    assert_eq!(passes_rule_2("1-3 b", "cdefg"), false);
    assert_eq!(passes_rule_2("2-9 c", "ccccccccc"), false);
}

#[test]
fn test_passes_rule_1() {
    assert_eq!(passes_rule_1("1-3 a", "abcde"), true);
    assert_eq!(passes_rule_1("1-3 a", "bcde"), false);
    assert_eq!(passes_rule_1("1-3 a", "aaa"), true);
    assert_eq!(passes_rule_1("1-3 a", "aaaa"), false);
}
