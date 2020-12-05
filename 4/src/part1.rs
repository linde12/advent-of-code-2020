#![feature(trait_alias)]
use std::collections::HashMap;

struct Passport<'a> {
    entries: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    fn new(raw: &'a str) -> Passport<'a> {
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

        Self { entries }
    }

    fn is_valid(&self) -> bool {
        self.entries.contains_key(&"ecl") && // Eye color
            self.entries.contains_key(&"pid") && // Passport ID
            self.entries.contains_key(&"eyr") && // Expiration year
            self.entries.contains_key(&"hcl") && // Hair color
            self.entries.contains_key(&"byr") && // Birth year
            self.entries.contains_key(&"iyr") && // Issue year
            self.entries.contains_key(&"hgt") // Height
    }
}

pub fn solve(input: &str) -> usize {
    let passports: Vec<Passport> = input
        .split("\n\n")
        .map(|raw_passport| Passport::new(raw_passport))
        .collect();

    passports.iter().filter(|p| p.is_valid()).count()
}

#[test]
fn test_1() {
    let input = include_str!("./input_test.txt").trim();
    let result = solve(&input);
    assert_eq!(result, 2)
}
