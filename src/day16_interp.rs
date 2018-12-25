use std::io::{self, Read};
use std::fmt;

#[derive(Debug, Clone, Copy)]
enum Source {
    Reg,
    Imm,
}

struct Op {
    op: Box<dyn Fn(usize, usize) -> usize>,
    sources: (Source, Source),
    name: String,
}

impl Op {
    fn new(op: Box<dyn Fn(usize, usize) -> usize>, sources: (Source, Source), name: &str) -> Op {
        Op { op, sources, name: name.to_string() }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn do_op(op: &Op, regs: &mut Vec<usize>, src_a: usize, src_b: usize, dest: usize) {
    let a = match op.sources.0 {
        Source::Imm => src_a,
        Source::Reg => regs[src_a],
    };
    let b = match op.sources.1 {
        Source::Imm => src_b,
        Source::Reg => regs[src_b],
    };
    regs[dest] = (op.op)(a, b);
}

fn parse_arr(line: &str) -> Vec<usize> {
    line[line.find("[").unwrap() + 1..line.find("]").unwrap()]
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

// Before: [2, 1, 1, 0]
// 4 2 1 3
// After:  [2, 1, 1, 0]
// like: [10,   12,   15]
//       gtrr   gtri, eqir
fn main() {
    // Define base operators.
    let mut ops = Vec::new();
    let arith_srcs = vec![(Source::Reg, Source::Reg), (Source::Reg, Source::Imm)];
    let mut cmp_srcs = arith_srcs.to_vec();
    cmp_srcs.push((Source::Imm, Source::Reg));
    for srcs in arith_srcs {
        ops.push(Op::new(Box::new(|a, b| a + b), srcs, "+"));
        ops.push(Op::new(Box::new(|a, b| a * b), srcs, "*"));
        ops.push(Op::new(Box::new(|a, b| a & b), srcs, "&"));
        ops.push(Op::new(Box::new(|a, b| a | b), srcs, "|"));
    }
    for srcs in vec![(Source::Reg, Source::Reg), (Source::Imm, Source::Reg)] {
        ops.push(Op::new(Box::new(|a, _b| a), srcs, "="));
    }
    for srcs in cmp_srcs {
        ops.push(Op::new(Box::new(|a, b| (a > b).into()), srcs, ">"));
        ops.push(Op::new(Box::new(|a, b| (a == b).into()), srcs, "=="));
    }
    assert_eq!(ops.len(), 16);

    // At the start, each number could be any possible op. (Index into |ops|)
    // let mut possible_op_assignment: Vec<Vec<usize>>= vec![(0..16).collect(); 16];

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.trim().split("\n");
    let mut like_three = 0;
    while let Some(before) = lines.next() {
        if !before.starts_with("Before:") {
            // Finished this section of input.
            break;
        }
        let regs = parse_arr(before);
        let op_line = lines.next().unwrap();
        let instr = op_line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let after = lines.next().unwrap();
        let after_regs = parse_arr(after);

        let mut i = 0;
        let mut possible_ops = (0..16).collect::<Vec<_>>(); //&mut possible_op_assignment[instr[0]];
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
        if possible_ops.len() >= 3 {
            like_three += 1;
        }

        // Skip next blank line
        lines.next();
    }

    println!("{}", like_three);
}
