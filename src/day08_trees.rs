use std::io::{self, Read};

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
    // Iterator data.
    meta_idx: usize,
    child_idx: usize,
}

impl Node {
    pub fn new(tree: &[i32]) -> Node {
        Node::new_helper(tree, 0).0
    }
    fn new_helper(tree: &[i32], mut idx: usize) -> (Node, usize) {
        let num_child = tree[idx];
        let num_metadata: usize = tree[idx + 1] as usize;
        let mut out = Node {
            children: vec![],
            metadata: vec![],
            meta_idx: 0,
            child_idx: 0,
        };
        idx += 2;
        for _ in 0..num_child {
            let tup = Node::new_helper(tree, idx);
            out.children.push(tup.0);
            idx = tup.1;
        }
        out.metadata.extend(tree[idx..idx + num_metadata].iter());
        (out, idx + num_metadata)
    }
    fn value(&self) -> i32 {
        if self.children.len() == 0 {
            self.metadata.iter().sum()
        } else {
            let mut sum = 0;
            for idx in &self.metadata {
                if 1 <= *idx && *idx <= self.children.len() as i32 {
                    sum += self.children[(*idx - 1) as usize].value();
                }
            }
            sum
        }
    }
}

impl Iterator for Node {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.meta_idx < self.metadata.len() {
            self.meta_idx += 1;
            return Some(self.metadata[self.meta_idx - 1]);
        } else {
            while self.child_idx < self.children.len() {
                if let Some(i) = self.children[self.child_idx].next() {
                    return Some(i);
                }
                self.child_idx += 1;
            }
        }
        None
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let ints_res = input
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let tree = Node::new(&ints_res);
    // Uncomment for part 1
    // println!("{}", tree.fold(0, |acc, x| acc + x));
    println!("{}", tree.value());
}
