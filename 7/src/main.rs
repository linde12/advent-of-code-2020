#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::{HashSet, HashMap};

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"^(.+) bags contain (.+)").unwrap();
    static ref CONTAINED_RE: Regex = Regex::new(r"^(\d+) (.+?) bag").unwrap();
}

mod part1;
mod part2;

pub fn parse_rules(input: &str) -> HashMap<String, HashSet<(usize, String)>> {
    let raw_rules: Vec<&str> = input.lines().collect();
    raw_rules.iter().filter_map(|raw| {
        if let Some(caps) = RULE_RE.captures(raw) {
            if caps.len() == 3 {
                let name = &caps[1];
                let raw_contained = &caps[2];

                let contained: HashSet<(usize, String)> = raw_contained.split(", ").filter_map(|raw| {
                    if let Some(caps) = CONTAINED_RE.captures(raw) {
                        if caps.len() == 3 {
                            let num: usize = caps[1].parse().unwrap_or(0);
                            return Some((num, caps[2].to_owned()))
                        }
                    }
                    None
                }).collect();

                return Some((name.to_owned(), contained));
            }
        }

        None
    }).collect()
}

fn main() {
    let input = include_str!("./input.txt").trim();

    // part 1
    let result = part1::solve(input);
    println!("part 1 solution: {}", result);

    // part 2
    let result = part2::solve(input);
    println!("part 2 solution: {}", result);
}
