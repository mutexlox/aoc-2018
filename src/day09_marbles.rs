use std::collections::VecDeque;
use std::io::{self, Read};

// Inserts |marble| into the marbles vec, updating |marbles|.
// Returns score delta.
fn insert_next(marbles: &mut VecDeque<u32>, marble: u32) -> u32 {
    if marble % 23 == 0 {
        for _ in 0..7 {
            let back = marbles.pop_back().unwrap();
            marbles.push_front(back);
        }
        let next_score = marbles.pop_back().unwrap();
        let front = marbles.pop_front().unwrap();
        marbles.push_back(front);
        marble + next_score
    } else {
        let front = marbles.pop_front().unwrap();
        marbles.push_back(front);
        marbles.push_back(marble);
        0
    }
}

fn get_high_score(num_players: usize, last: u32) -> u32 {
    let mut marbles = VecDeque::new();
    marbles.push_back(0);
    let mut scores = vec![0; num_players];
    let mut cur_player = 0;
    let mut score_max = 0;
    for i in 1..last + 1 {
        scores[cur_player] += insert_next(&mut marbles, i);
        if scores[cur_player] > score_max {
            score_max = scores[cur_player];
        }
        cur_player = (cur_player + 1) % num_players;
    }
    *scores.iter().max().unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let words = input.split_whitespace().collect::<Vec<_>>();
    let num_players = words[0].parse::<usize>().unwrap();
    let last_marble = words[6].parse::<u32>().unwrap();
    println!("{}", get_high_score(num_players, last_marble));
}
