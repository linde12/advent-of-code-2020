use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt").trim();
    let passports = parse_passports(input);

    // part 1
    let result = solve_1(&passports);
    println!("part 1 solution: {}", result);

    // part 2
    let result = solve_2(&passports);
    println!("part 2 solution: {}", result);
}

/// Parses a passport batch file into a vector of hashmaps.
fn parse_passports(input: &str) -> Vec<HashMap<String, String>> {
    input
        .split("\n\n")
        .map(|passport| {
            let passport = passport.replace('\n', " ");
            let mut map = HashMap::new();

            let key_values: Vec<(String, String)> = passport
                .trim()
                .split(" ")
                .filter_map(|kv| {
                    let kv: Vec<String> = kv.split(":").map(|s| s.to_owned()).collect();

                    let mut iter = kv.into_iter();
                    if let Some(key) = iter.next() {
                        if let Some(value) = iter.next() {
                            Some((key.to_string(), value))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            key_values.into_iter().for_each(|(key, value)| {
                map.insert(key.to_string(), value);
            });
            map
        })
        .collect()
}

/// Filters out all passports that don't contain all values and returns
/// a count of valid passports.
fn solve_1(passports: &Vec<HashMap<String, String>>) -> usize {
    passports
        .iter()
        .filter(|passport| {
            // Check whether this passport contains all required fields
            vec![
                passport.get("ecl"),
                passport.get("pid"),
                passport.get("eyr"),
                passport.get("hcl"),
                passport.get("byr"),
                passport.get("iyr"),
                passport.get("hgt"),
            ]
            .iter()
            .all(|opt| opt.is_some())
        })
        .count()
}

/// Filters out all passports that don't contain all values OR
/// don't pass their respective validation requirement. Returns
/// a count of valid passports.
fn solve_2(passports: &Vec<HashMap<String, String>>) -> usize {
    passports
        .iter()
        .filter(|passport| {
            // Map which value should be validated by which function
            let entries: Vec<&String> = vec![
                passport.get("ecl"), // Eye color
                passport.get("pid"), // Passport ID
                passport.get("eyr"), // Expiration year
                passport.get("hcl"), // Hair color
                passport.get("byr"), // Birth year
                passport.get("iyr"), // Issue year
                passport.get("hgt"), // Height
            ]
            .iter()
            .filter_map(|entry| *entry)
            .collect();

            // If there aren't still seven keys after we've filtered away `None`-values then that
            // means that some of the required keys are missing and the passport is invalid
            if entries.len() != 7 {
                return false;
            }

            // Validators to be zipped into validation_map
            let validators: Vec<fn(&str) -> bool> = vec![
                validate_eye_color,
                validate_passport_id,
                validate_expiration,
                validate_hair_color,
                validate_birth,
                validate_issue,
                validate_height,
            ];

            entries
                .iter()
                .zip(validators)
                .all(|(entry, validate)| validate(entry))
        })
        .count()
}

fn validate_passport_id(value: &str) -> bool {
    value.len() == 9
}

fn validate_eye_color(value: &str) -> bool {
    let valid_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid_colors.contains(&value)
}

fn validate_expiration(value: &str) -> bool {
    let n: Option<u32> = value.parse().ok();
    if let Some(n) = n {
        n >= 2020 && n <= 2030
    } else {
        false
    }
}

fn validate_hair_color(value: &str) -> bool {
    let valid_hex = value.chars().skip(1).all(|c| match c {
        '0'..='9' | 'a'..='f' => true,
        _ => false,
    });

    if let Some(first) = value.chars().nth(0) {
        first == '#' && valid_hex && value.len() == 7
    } else {
        false
    }
}

fn validate_birth(value: &str) -> bool {
    let n: Option<u32> = value.parse().ok();
    if let Some(n) = n {
        n >= 1920 && n <= 2002
    } else {
        false
    }
}

fn validate_issue(value: &str) -> bool {
    let n: Option<u32> = value.parse().ok();
    if let Some(n) = n {
        n >= 2010 && n <= 2020
    } else {
        false
    }
}

fn validate_height(value: &str) -> bool {
    if value.len() < 3 {
        return false;
    }

    let height: String = value.chars().take(value.len() - 2).collect();
    let suffix: String = value.chars().skip(value.len() - 2).collect();
    let height: Option<u32> = height.parse().ok();

    if let Some(height) = height {
        match suffix.as_str() {
            "cm" => height >= 150 && height <= 193,
            "in" => height >= 59 && height <= 76,
            _ => false,
        }
    } else {
        false
    }
}

#[test]
fn test_1() {
    let input = include_str!("./input_test.txt").trim();
    let passports = parse_passports(input);
    let result = solve_1(&passports);
    assert_eq!(result, 2)
}

#[test]
fn test_2_invalid() {
    let input = include_str!("./input_invalid_passports.txt").trim();
    let passports = parse_passports(input);
    let result = solve_2(&passports);
    assert_eq!(result, 0)
}

#[test]
fn test_2_valid() {
    let input = include_str!("./input_valid_passports.txt").trim();
    let passports = parse_passports(input);
    let result = solve_2(&passports);
    assert_eq!(result, 4)
}

#[test]
fn test_validate_height() {
    assert_eq!(validate_height("150cm"), true);
    assert_eq!(validate_height("149cm"), false);
    assert_eq!(validate_height("193cm"), true);
    assert_eq!(validate_height("194cm"), false);
    assert_eq!(validate_height("150"), false);
}

#[test]
fn test_validate_eye_color() {
    assert_eq!(validate_eye_color("blu"), true);
    assert_eq!(validate_eye_color("purple"), false);
}

#[test]
fn test_validate_issue() {
    assert_eq!(validate_issue("2010"), true);
    assert_eq!(validate_issue("2009"), false);
    assert_eq!(validate_issue("2020"), true);
    assert_eq!(validate_issue("2021"), false);
}

#[test]
fn test_validate_birth() {
    assert_eq!(validate_birth("1920"), true);
    assert_eq!(validate_birth("1919"), false);
    assert_eq!(validate_birth("2002"), true);
    assert_eq!(validate_birth("2003"), false);
}

#[test]
fn test_validate_hair_color() {
    assert_eq!(validate_hair_color("#fffaaa"), true);
    assert_eq!(validate_hair_color("#fafafa"), true);
    assert_eq!(validate_hair_color("#fafafaf"), false);
    assert_eq!(validate_hair_color("#000000"), true);
    assert_eq!(validate_hair_color("#123456"), true);
    assert_eq!(validate_hair_color("#123abc"), true);
    assert_eq!(validate_hair_color("123abc"), false);
}

#[test]
fn test_validate_expiration() {
    assert_eq!(validate_expiration("2020"), true);
    assert_eq!(validate_expiration("2019"), false);
    assert_eq!(validate_expiration("2030"), true);
    assert_eq!(validate_expiration("2031"), false);
}
