mod interp;

use crate::interp::{do_op, get_ops};
use std::error;
use std::fmt;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug)]
struct Instruction {
    name: String,
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, Clone)]
struct FormatError;

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

fn main() {
    let ops = get_ops();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.trim().split("\n");
    let ip_reg = lines
        .next()
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let instrs = lines
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();
    let mut regs = vec![0; 6];
    let mut ip = 0;
    while ip < instrs.len() {
        regs[ip_reg] = ip;

        let inst = &instrs[ip];
        do_op(&ops[&inst.name], &mut regs, inst.a, inst.b, inst.c);
        ip = regs[ip_reg];
        ip += 1;
    }
    println!("{}", regs[0]);
}
