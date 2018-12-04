use std::io::{self, Read};
use std::collections::HashMap;

fn histogram(s : &str) -> HashMap<char, i32> {
    let mut out = HashMap::<char, i32>::new();
    for c in s.chars() {
        let i = out.entry(c).or_insert(0);
        *i += 1
    }
    out
}

fn has_letter_n_times(hist : &HashMap<char, i32>, n : i32) -> bool {
    hist.values().any(|x| *x == n)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let ids = input.split_whitespace();
    let mut num_with_pairs = 0;
    let mut num_with_triples = 0;
    for id in ids {
        let hist = histogram(id);
        if has_letter_n_times(&hist, 2) {
            num_with_pairs += 1;
        }
        if has_letter_n_times(&hist, 3) {
            num_with_triples += 1;
        }
    }
    println!("{}", num_with_pairs * num_with_triples);
}
