extern crate regex;
use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};

// Called when work on a node is complete. Removes node from graph.
fn finish_node(
    node: &str,
    graph: &mut HashMap<String, Vec<String>>,
    rev_graph: &mut HashMap<String, Vec<String>>,
    no_deps: &mut HashSet<String>) {
    no_deps.remove(node);
    let neighbors = graph.get(node).unwrap_or(&vec![]).clone();
    for m in neighbors {
        let mut n_entry = graph.entry(node.to_string());
        let mut m_entry = rev_graph
            .entry(m.to_string())
            .and_modify(|v| v.retain(|x| *x != node));
        // If there are now no incoming edges to m, delete from rev_graph.
        if let Entry::Occupied(o) = m_entry {
            if o.get().len() == 0 {
                o.remove();
                no_deps.insert(m.to_string());
            }
        }
        // Remove processed edge.
        n_entry.and_modify(|v| v.retain(|x| *x != m));
    }
}

fn part_1(
    mut graph: HashMap<String, Vec<String>>,
    mut rev_graph: HashMap<String, Vec<String>>,
    mut no_deps: HashSet<String>,
    mut next_nodes: Vec<String>
) -> String {
    // Build up exploration order.
    let mut order = Vec::new();
    while let Some(n) = next_nodes.pop() {
        finish_node(&n, &mut graph, &mut rev_graph, &mut no_deps);
        next_nodes.clear();
        next_nodes.extend(no_deps.iter().cloned());
        next_nodes.sort_unstable_by(|a, b| b.cmp(a));

        order.push(n);
    }

    order.iter().cloned().collect()
}

// Return how long it takes for a given node, based on each node taking 60
// seconds plus the int corresponding to its letter. e.g. A takes 61 seconds,
// B takes 62.
fn time_for_node(n: u8) -> u32 {
    (61 + n - ('A' as u8)).into()
}

// Like part 1, but with parallelism and operations taking time.
// Returns amount of time taken.
fn part_2(mut graph: HashMap<String, Vec<String>>,
          mut rev_graph: HashMap<String, Vec<String>>,
          mut no_deps: HashSet<String>,
          mut next_nodes: Vec<String>,
          num_workers: usize) -> u32 {
    // Track what each worker is currently working on and when it'll be done.
    let mut worker_track : Vec<Option<(String, u32)>> = vec![None; num_workers];

    let mut time : u32 = 0;
    while !no_deps.is_empty() {
        for i in 0..worker_track.len() {
            if let Some((n, t)) = worker_track[i].clone() {
                if t <= time {
                    finish_node(&n, &mut graph, &mut rev_graph, &mut no_deps);
                    // Add anything with no deps that is not already worked on.
                    for x in no_deps.iter() {
                        if worker_track.iter().cloned().filter_map(|y| y).all(|(n, _)| n != *x) {
                            next_nodes.push(x.to_string());
                        }
                    }
                    next_nodes.sort_unstable_by(|a, b| b.cmp(a));
                    next_nodes.dedup();
                    worker_track[i] = None;
                }
            }
        }
        while let Some(w_idx) = worker_track.iter().position(|x| x.is_none()) {
            // If a worker is available, get it some work.
            if let Some(n) = next_nodes.pop() {
                worker_track[w_idx] = Some((n.clone(), time + time_for_node(n.as_bytes()[0])));
            } else {
                break;
            }
        }
        time += 1;
    }
    time - 1
}

fn main() {
    let dep_re = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let deps = input.trim().split("\n");

    let mut graph = HashMap::<String, Vec<String>>::new();
    //  graph in reverse (Node -> List of incoming nodes).
    let mut rev_graph = HashMap::<String, Vec<String>>::new();
    for dep in deps {
        let nodes = dep_re.captures(&dep).unwrap();
        let src = nodes.get(1).unwrap().as_str();
        let dest = nodes.get(2).unwrap().as_str();

        graph
            .entry(src.to_string())
            .and_modify(|v| v.push(dest.to_string()))
            .or_insert(vec![dest.to_string()]);
        rev_graph
            .entry(dest.to_string())
            .and_modify(|v| v.push(src.to_string()))
            .or_insert(vec![src.to_string()]);
    }

    let nodes = graph.keys().cloned().collect::<HashSet<String>>();
    let incoming = rev_graph.keys().cloned().collect::<HashSet<String>>();
    // Set of nodes with no incoming.
    let no_deps: HashSet<_> = nodes.difference(&incoming).cloned().collect();

    let mut next_nodes: Vec<_> = no_deps.iter().cloned().collect();
    // Sort in reverse so smallest is at end.
    next_nodes.sort_unstable_by(|a, b| b.cmp(a));

    println!("{}", part_1(graph.clone(), rev_graph.clone(), no_deps.clone(), next_nodes.clone()));
    println!("{}", part_2(graph, rev_graph, no_deps, next_nodes, 5));
}
