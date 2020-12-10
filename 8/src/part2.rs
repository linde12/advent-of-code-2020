use std::collections::HashSet;

fn execute(instr: &crate::Instruction, pc: usize, acc: &mut isize) -> usize {
    match instr.op {
        "acc" => {
            *acc = acc.wrapping_add(instr.arg);
            pc + 1
        }
        "jmp" => (pc as isize + instr.arg) as usize,
        _ => pc.wrapping_add(1),
    }
}

pub fn solve(input: &str) -> isize {
    let mut prev_encountered = 0;

    'outer: loop {
        let mut visited = HashSet::new();
        let mut instructions = crate::parse_instructions(input);
        let mut pc = 0;
        let mut acc: isize = 0;
        let mut nopsjmps_encountered = 0;

        while pc < instructions.len() {
            if visited.contains(&pc) {
                prev_encountered += 1;
                continue 'outer;
            }
            visited.insert(pc);

            let instr = &mut instructions[pc];

            if instr.op == "jmp" || instr.op == "nop" {
                if nopsjmps_encountered == prev_encountered {
                    if instr.op == "jmp" {
                        instr.op = "nop"
                    } else if instr.op == "nop" {
                        instr.op = "jmp"
                    }
                }
                nopsjmps_encountered += 1;
            }

            pc = execute(instr, pc, &mut acc);
        }

        return acc;
    }
}

#[test]
fn test_2() {
    let input = include_str!("./input_test2.txt").trim();
    let result = solve(&input);
    assert_eq!(result, 8)
}
