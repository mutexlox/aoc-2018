use std::io::{self, Read};

// Return Some(idx) if haystack is found at idx.
fn add_new_scores(scores: &mut Vec<u32>, elves: &Vec<usize>, haystack: &[u32]) -> Option<usize> {
    let sum = scores[elves[0]] + scores[elves[1]];
    let mut ret = None;
    if sum >= 10 {
        scores.push((sum / 10) % 10);
        if scores.ends_with(haystack) {
            ret = Some(scores.len() - haystack.len());
        }
    }
    scores.push(sum % 10);
    if scores.ends_with(haystack) {
        ret = Some(scores.len() - haystack.len());
    }
    ret
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let after = input.trim().parse().unwrap();

    let digits = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut scores = vec![3, 7];
    let mut elf_indices = vec![0, 1];
    loop {
        //while scores.len() < after + 10 {
        let idx_opt = add_new_scores(&mut scores, &elf_indices, digits.as_slice());
        for idx in &mut elf_indices {
            *idx = (*idx + 1 + (scores[*idx] as usize)) % scores.len();
        }
        if let Some(idx) = idx_opt {
            println!("{}", idx);
            break;
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
