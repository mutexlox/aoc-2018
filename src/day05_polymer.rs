use std::io::{self, Read};

fn is_reactable(a: char, b: char) -> bool {
    a.to_lowercase().to_string() == b.to_lowercase().to_string() && a != b
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let alpha = "abcdefghijklmnopqrstuvwxyz";

    let mut min_len = 0;
    for remove in alpha.chars() {
        let mut stack = Vec::new();
        for c in input.trim().chars() {
            if c.to_lowercase().to_string() != remove.to_lowercase().to_string() {
                if stack.len() != 0 && is_reactable(stack[stack.len() - 1], c) {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            }
        }
        if stack.len() < min_len || remove == 'a' {
            min_len = stack.len();
        }
    }
    println!("Min len: {}", min_len);
}
