extern crate regex;
use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};

fn part_1(
    mut graph: HashMap<String, Vec<String>>,
    mut rev_graph: HashMap<String, Vec<String>>,
) -> String {
    let nodes = graph.keys().cloned().collect::<HashSet<String>>();
    let incoming = rev_graph.keys().cloned().collect::<HashSet<String>>();
    // Set of nodes with no incoming.
    let mut no_deps: HashSet<String> = nodes.difference(&incoming).cloned().collect();

    let mut next_nodes = no_deps.iter().cloned().collect::<Vec<String>>();
    // Sort in reverse so smallest is at end.
    next_nodes.sort_unstable_by(|a, b| b.cmp(a));

    // Build up exploration order.
    let mut order = Vec::new();
    while let Some(n) = next_nodes.pop() {
        no_deps.remove(&n);
        let neighbors = graph.get(&n).unwrap_or(&vec![]).clone();
        for m in neighbors {
            let mut n_entry = graph.entry(n.to_string());
            let mut m_entry = rev_graph
                .entry(m.to_string())
                .and_modify(|v| v.retain(|x| *x != n));
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
        next_nodes = no_deps.iter().cloned().collect();
        next_nodes.sort_unstable_by(|a, b| b.cmp(a));
        order.push(n);
    }

    order.iter().cloned().collect()
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

    println!("{}", part_1(graph, rev_graph));
}
