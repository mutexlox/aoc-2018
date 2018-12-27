mod interp;

use crate::interp::{do_op, get_ops, Op};
use std::collections::HashMap;
use std::io::{self, Read};

fn parse_arr(line: &str) -> Vec<usize> {
    line[line.find("[").unwrap() + 1..line.find("]").unwrap()]
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

/// Build a mapping from operators in the input to operators in our order.
fn get_op_mapping<'a, I>(ops: &HashMap<String, Op>, mut sample_input: I) -> Vec<&String>
where
    I: Iterator<Item = &'a str>,
{
    // At the start, each number could be any possible op. (Index into |ops|)
    let mut possible_op_assignment: Vec<Vec<&String>> = vec![ops.keys().collect(); 16];

    while let Some(before) = sample_input.next() {
        if !before.starts_with("Before:") {
            // Finished this section of input.
            break;
        }
        let regs = parse_arr(before);
        let op_line = sample_input.next().unwrap();
        let instr = op_line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let after = sample_input.next().unwrap();
        let after_regs = parse_arr(after);

        let mut i = 0;
        let possible_ops = &mut possible_op_assignment[instr[0]];
        while i < possible_ops.len() {
            let op = &ops[possible_ops[i]];
            let mut tmp_regs = regs.to_vec();
            do_op(op, &mut tmp_regs, instr[1], instr[2], instr[3]);
            if after_regs != tmp_regs {
                possible_ops.remove(i);
            } else {
                i += 1;
            }
        }
        // Skip next blank line
        sample_input.next();
    }

    // Filter down to unique assignments.
    let mut unique = possible_op_assignment
        .iter()
        .cloned()
        .filter(|poss| poss.len() == 1)
        .collect::<Vec<_>>();
    while unique.len() < 16 {
        for other in &mut possible_op_assignment {
            if other.len() > 1 {
                other.retain(|x| !unique.contains(&vec![*x]));
            }
        }
        unique = possible_op_assignment
            .iter()
            .cloned()
            .filter(|poss| poss.len() == 1)
            .collect();
    }

    possible_op_assignment.iter().cloned().flatten().collect()
}

fn main() {
    // Define base operators.
    let ops = get_ops();
    assert_eq!(ops.len(), 16);

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.trim().split("\n\n\n\n").collect::<Vec<_>>();

    let op_mapping = get_op_mapping(&ops, lines[0].split("\n"));

    let mut regs = vec![0; 4];

    for instr in lines[1].split("\n") {
        let parsed_instr: Vec<usize> = instr
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let op_idx = op_mapping[parsed_instr[0]];
        do_op(
            &ops[op_idx],
            &mut regs,
            parsed_instr[1],
            parsed_instr[2],
            parsed_instr[3],
        );
    }
    println!("{}", regs[0]);
}
