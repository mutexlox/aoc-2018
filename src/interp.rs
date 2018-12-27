use std::fmt;

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
