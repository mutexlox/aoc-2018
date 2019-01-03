mod interp;

use crate::interp::Program;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut prog = input.parse::<Program>().unwrap();
    prog.regs = vec![0; 6];
    prog.regs[0] = 1;
    prog.exec(Some(1));
    let upper = prog.regs[5];
    let mut count = 0;
    // This is an optimized version of the program in inputs/day19_interp.txt
    for i in 1..upper + 1 {
        if prog.regs[5] % i == 0 {
            count += i;
        }
    }
    println!("{}", count);
}
