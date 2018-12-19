use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.trim().split("\n").collect::<Vec<_>>();
    let initial = lines[0].split(":").nth(1).unwrap().trim();

    let mut state = HashSet::<i32>::new();
    let mut i = 0;
    for c in initial.chars() {
        if c == '#' {
            state.insert(i);
        }
        i += 1;
    }

    let mut rules = HashSet::<i32>::new();
    for rule_line in &lines[2..] {
        let rule = rule_line.split("=>").map(|s| s.trim()).collect::<Vec<_>>();
        let mut i = 0;
        for c in rule[0].chars() {
            i = i << 1;
            if c == '#' {
                i = i + 1;
            }
        }
        if rule[1] == "#" {
            rules.insert(i);
        }
    }

    let mut min_set: i32 = 0;
    let mut max_set: i32 = initial.len() as i32;
    let mut new_state = HashSet::<i32>::new();
    let upper = 128;
    for _ in 0..upper {
        new_state.clear();
        let mut new_min_set = std::i32::MAX;
        let mut new_max_set = std::i32::MIN;
        assert!((max_set + 3) - (min_set - 2) < 500);
        for i in (min_set - 2)..(max_set + 3) {
            let mut rule = 0;
            for j in (i - 2) .. (i + 3) {
                rule = rule << 1;
                if state.contains(&j) {
                    rule += 1;
                }
            }
            if rules.contains(&rule) {
                new_state.insert(i);
                if i < new_min_set {
                    new_min_set = i;
                }
                if i > new_max_set {
                    new_max_set = i;
                }
            }
        }
        min_set = new_min_set;
        max_set = new_max_set;
        state = new_state.clone();
    }
    let len = state.len();
    let sum: i32 = state.iter().sum();
    println!("{}", sum);
    println!("{}", sum as usize + (50_000_000_000 - upper) * len);
}
