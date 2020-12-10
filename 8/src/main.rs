mod part1;
mod part2;

#[derive(Debug)]
pub struct Instruction<'a> {
    pub op: &'a str,
    pub arg: isize,
}

impl <'a>Instruction<'a> {
    fn new(op: &'a str, arg: isize) -> Self {
        Self { op, arg }
    }
}

fn parse_instructions<'a>(input: &'a str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let op = parts[0];
                let sign = parts[1].chars().nth(0);
                let num: Option<isize> = parts[1].chars().skip(1).collect::<String>().parse().ok();
                if let Some(num) = num {
                    match sign {
                        Some('+') => Some(Instruction::new(op, num)),
                        Some('-') => Some(Instruction::new(op, 0 - num)),
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
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
