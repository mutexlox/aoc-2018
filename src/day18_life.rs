use std::collections::HashMap;
use std::io::{self, Read};

/// Returns count of all adjacent chars.
fn neighbor_count(area: &Vec<Vec<char>>, i: usize, j: usize) -> HashMap<char, usize> {
    let mut count = HashMap::new();
    for di in -1..2 {
        for dj in -1..2 {
            if di == 0 && dj == 0 {
                continue;
            }
            let new_i = i as i32 + di;
            let new_j = j as i32 + dj;
            if new_i >= 0 && (new_i as usize) < area.len() {
                if new_j >= 0 && (new_j as usize) < area[new_i as usize].len() {
                    count
                        .entry(area[new_i as usize][new_j as usize])
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }
    }
    count
}

fn step(area: &mut Vec<Vec<char>>) {
    let old = area.to_vec();
    for i in 0..old.len() {
        for j in 0..old[i].len() {
            let neigh = neighbor_count(&old, i, j);
            if old[i][j] == '.' {
                if let Some(trees) = neigh.get(&'|') {
                    if *trees >= 3 {
                        area[i][j] = '|';
                    }
                }
            } else if old[i][j] == '|' {
                if let Some(lumber) = neigh.get(&'#') {
                    if *lumber >= 3 {
                        area[i][j] = '#';
                    }
                }
            } else {
                assert!(old[i][j] == '#');
                area[i][j] = '.';
                if let Some(lumber) = neigh.get(&'#') {
                    if let Some(trees) = neigh.get(&'|') {
                        if *lumber > 0 && *trees > 0 {
                            area[i][j] = '#';
                        }
                    }
                }
            }
        }
    }
}

/// Return number of wooded areas * number of lumber yards.
fn resource_count(area: &Vec<Vec<char>>) -> usize {
    let wooded: usize = area
        .iter()
        .map(|l| l.iter().filter(|c| **c == '|').count())
        .sum();
    let lumber: usize = area
        .iter()
        .map(|l| l.iter().filter(|c| **c == '#').count())
        .sum();
    wooded * lumber
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input
        .trim()
        .split("\n")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    for _ in 0..10 {
        step(&mut lines);
    }
    println!("{}", resource_count(&lines));
}
