use std::collections::HashSet;

fn execute(instr: &crate::Instruction, pc: usize, acc: &mut isize) -> usize {
    match instr.op {
        "acc" => { *acc = acc.wrapping_add(instr.arg); pc + 1 },
        "jmp" => (pc as isize + instr.arg) as usize,
        _ => pc.wrapping_add(1),
    }
}

pub fn solve(input: &str) -> isize {
    let mut visited = HashSet::new();
    let instructions = crate::parse_instructions(input);
    let mut pc = 0;
    let mut acc: isize = 0;

    while pc < instructions.len() {
        if visited.contains(&pc) {
            return acc;
        }
        visited.insert(pc);

        let instr = &instructions[pc];
        pc = execute(instr, pc, &mut acc);
    }

    0
}

#[test]
fn test_1() {
    let input = include_str!("./input_test.txt").trim();
    let result = solve(&input);
    assert_eq!(result, 5)
}
