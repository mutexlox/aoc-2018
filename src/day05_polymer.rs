use std::io::{self, Read};

fn is_unit(a: char, b: char) -> bool {
    a.to_lowercase().to_string() == b.to_lowercase().to_string() && a != b
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut stack = Vec::new();
    for c in input.trim().chars() {
        if stack.len() != 0 && is_unit(stack[stack.len() - 1], c) {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    println!("Remaining: {}", stack.len());
}
