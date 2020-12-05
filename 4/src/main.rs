#![feature(trait_alias)]
use std::collections::HashMap;

trait ValidatorFn = Fn(&HashMap<&str, &str>) -> bool;

#[derive(Debug)]
struct Passport<'a, F: ValidatorFn> {
    entries: HashMap<&'a str, &'a str>,
    validator: F,
}

impl<'a, F: ValidatorFn> Passport<'a, F> {
    fn new(raw: &'a str, validator: F) -> Passport<'a, F> {
        let entries = raw
            .split_whitespace()
            .filter_map(|kv| {
                let kv: Vec<&str> = kv.split(":").collect();
                let key = kv.iter().nth(0)?;
                if let Some(value) = kv.iter().nth(1) {
                    Some((*key, *value))
                } else {
                    None
                }
            })
            .collect();

        Self { entries, validator }
    }

    fn is_valid(&self) -> bool {
        (self.validator)(&self.entries)
    }
}

fn main() {
    let input = include_str!("./input.txt").trim();

    // part 1
    let result = solve_1(&input);
    println!("part 1 solution: {}", result);

    // part 2
    let result = solve_2(&input);
    println!("part 2 solution: {}", result);
}

fn solve_1(input: &str) -> usize {
    let validate = |map: &HashMap<&str, &str>| {
        map.contains_key(&"ecl") && // Eye color
            map.contains_key(&"pid") && // Passport ID
            map.contains_key(&"eyr") && // Expiration year
            map.contains_key(&"hcl") && // Hair color
            map.contains_key(&"byr") && // Birth year
            map.contains_key(&"iyr") && // Issue year
            map.contains_key(&"hgt") // Height
    };

    // Passport::new(&"abc", validate);
    let passports: Vec<Passport<_>> = input
        .split("\n\n")
        .map(|raw_passport| Passport::new(raw_passport, validate))
        .collect();

    passports.iter().filter(|p| p.is_valid()).count()
}

fn solve_2(input: &str) -> usize {
    let validate = |map: &HashMap<&str, &str>| {
        let has_all_keys = map.contains_key(&"ecl") && // Eye color
            map.contains_key(&"pid") && // Passport ID
            map.contains_key(&"eyr") && // Expiration year
            map.contains_key(&"hcl") && // Hair color
            map.contains_key(&"byr") && // Birth year
            map.contains_key(&"iyr") && // Issue year
            map.contains_key(&"hgt"); // Height

        if !has_all_keys {
            return false;
        }

        validate_eye_color(map.get(&"ecl").unwrap_or(&""))
            && validate_passport_id(map.get(&"pid").unwrap_or(&""))
            && validate_expiration(map.get(&"eyr").unwrap_or(&""))
            && validate_hair_color(map.get(&"hcl").unwrap_or(&""))
            && validate_birth(map.get(&"byr").unwrap_or(&""))
            && validate_issue(map.get(&"iyr").unwrap_or(&""))
            && validate_height(map.get(&"hgt").unwrap_or(&""))
    };

    let passports: Vec<Passport<_>> = input
        .split("\n\n")
        .map(|raw_passport| Passport::new(raw_passport, validate))
        .collect();

    passports.iter().filter(|p| p.is_valid()).count()
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

    let (height, suffix) = value.split_at(value.len() - 2);
    let height: Option<u32> = height.parse().ok();

    if let Some(height) = height {
        match suffix {
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
    let result = solve_1(&input);
    assert_eq!(result, 2)
}

#[test]
fn test_2_invalid() {
    let input = include_str!("./input_invalid_passports.txt").trim();
    let result = solve_2(&input);
    assert_eq!(result, 0)
}

#[test]
fn test_2_valid() {
    let input = include_str!("./input_valid_passports.txt").trim();
    let result = solve_2(&input);
    assert_eq!(result, 4)
}

#[test]
fn test_validate_height() {
    assert_eq!(validate_height(""), false);
    assert_eq!(validate_height("150cm"), true);
    assert_eq!(validate_height("149cm"), false);
    assert_eq!(validate_height("193cm"), true);
    assert_eq!(validate_height("194cm"), false);
    assert_eq!(validate_height("150"), false);
}

#[test]
fn test_validate_eye_color() {
    assert_eq!(validate_height(""), false);
    assert_eq!(validate_eye_color("blu"), true);
    assert_eq!(validate_eye_color("purple"), false);
}

#[test]
fn test_validate_issue() {
    assert_eq!(validate_height(""), false);
    assert_eq!(validate_issue("2010"), true);
    assert_eq!(validate_issue("2009"), false);
    assert_eq!(validate_issue("2020"), true);
    assert_eq!(validate_issue("2021"), false);
}

#[test]
fn test_validate_birth() {
    assert_eq!(validate_height(""), false);
    assert_eq!(validate_birth("1920"), true);
    assert_eq!(validate_birth("1919"), false);
    assert_eq!(validate_birth("2002"), true);
    assert_eq!(validate_birth("2003"), false);
}

#[test]
fn test_validate_hair_color() {
    assert_eq!(validate_height(""), false);
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
    assert_eq!(validate_height(""), false);
    assert_eq!(validate_expiration("2020"), true);
    assert_eq!(validate_expiration("2019"), false);
    assert_eq!(validate_expiration("2030"), true);
    assert_eq!(validate_expiration("2031"), false);
}
