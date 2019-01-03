mod interp;

use crate::interp::{get_ops, Instruction};
use std::io::{self, Read};

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
    regs[0] = 1;
    while ip < instrs.len() && ip != 1 {
        regs[ip_reg] = ip;

        let inst = &instrs[ip];
        inst.execute(&ops, &mut regs);
        ip = regs[ip_reg];
        ip += 1;
    }
    let upper = (regs[5] as f64).sqrt() as usize;
    let mut count = 0;
    // This is an optimized version of the program in inputs/day19_interp.txt
    for i in 1..upper + 1 {
        if regs[5] % i == 0 {
            count += i;
            let j = regs[5] / i;
            if j != i {
                count += j;
            }
        }
    }
    println!("{}", count);
}
