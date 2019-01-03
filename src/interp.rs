use std::collections::HashMap;
use std::error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct Instruction {
    name: String,
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, Clone)]
pub struct FormatError;

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid format for input; expect name a b c")
    }
}

impl error::Error for FormatError {
    fn description(&self) -> &str {
        "invalid format for input; expect name a b c"
    }
    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl FromStr for Instruction {
    type Err = FormatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        let a = parts[1].parse::<usize>().or(Err(FormatError))?;
        let b = parts[2].parse::<usize>().or(Err(FormatError))?;
        let c = parts[3].parse::<usize>().or(Err(FormatError))?;
        Ok(Instruction {
            name: parts[0].to_string(),
            a,
            b,
            c,
        })
    }
}

impl Instruction {
    pub fn execute(&self, ops: &HashMap<String, Op>, regs: &mut Vec<usize>) {
        do_op(&ops[&self.name], regs, self.a, self.b, self.c);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Source {
    Reg,
    Imm,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Source::Reg => write!(f, "r"),
            Source::Imm => write!(f, "i"),
        }
    }
}

pub struct Op {
    op: Box<dyn Fn(usize, usize) -> usize>,
    sources: (Source, Source),
    name: String,
}

impl Op {
    pub fn new(
        op: Box<dyn Fn(usize, usize) -> usize>,
        sources: (Source, Source),
        name: String,
    ) -> Op {
        Op { op, sources, name }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn do_op(op: &Op, regs: &mut Vec<usize>, src_a: usize, src_b: usize, dest: usize) {
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

pub fn get_ops() -> HashMap<String, Op> {
    let mut ops = HashMap::new();
    let arith_srcs = vec![(Source::Reg, Source::Reg), (Source::Reg, Source::Imm)];
    let mut cmp_srcs = arith_srcs.to_vec();
    cmp_srcs.push((Source::Imm, Source::Reg));
    for srcs in arith_srcs {
        let add = Op::new(Box::new(|a, b| a + b), srcs, format!("add{}", srcs.1));
        ops.insert(add.name.clone(), add);
        let mul = Op::new(Box::new(|a, b| a * b), srcs, format!("mul{}", srcs.1));
        ops.insert(mul.name.clone(), mul);
        let ban = Op::new(Box::new(|a, b| a & b), srcs, format!("ban{}", srcs.1));
        ops.insert(ban.name.clone(), ban);
        let bor = Op::new(Box::new(|a, b| a | b), srcs, format!("bor{}", srcs.1));
        ops.insert(bor.name.clone(), bor);
    }
    for srcs in vec![(Source::Reg, Source::Imm), (Source::Imm, Source::Imm)] {
        let set = Op::new(Box::new(|a, _b| a), srcs, format!("set{}", srcs.0));
        ops.insert(set.name.clone(), set);
    }
    for srcs in cmp_srcs {
        let gt = Op::new(
            Box::new(|a, b| (a > b).into()),
            srcs,
            format!("gt{}{}", srcs.0, srcs.1),
        );
        ops.insert(gt.name.clone(), gt);
        let eq = Op::new(
            Box::new(|a, b| (a == b).into()),
            srcs,
            format!("eq{}{}", srcs.0, srcs.1),
        );
        ops.insert(eq.name.clone(), eq);
    }
    assert_eq!(ops.len(), 16);
    ops
}
