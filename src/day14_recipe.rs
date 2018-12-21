use std::io::{self, Read};

fn add_new_scores(scores: &mut Vec<usize>, elves: &Vec<usize>) {
    let sum: usize = scores[elves[0]] + scores[elves[1]];
    if sum >= 10 {
        scores.push((sum / 10) % 10);
    }
    scores.push(sum % 10);
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let after = input.trim().parse().unwrap();

    let mut scores = vec![3, 7];
    let mut elf_indices = vec![0, 1];
    while scores.len() < after + 10 {
        add_new_scores(&mut scores, &elf_indices);
        for idx in &mut elf_indices {
            *idx = (*idx + 1 + scores[*idx]) % scores.len();
        }
    }
    println!(
        "{}",
        scores[after..after + 10]
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("")
    );
}
