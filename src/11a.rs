// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
#[macro_use]
extern crate scan_rules;
use scan_rules::{
    scan,
    scanner::{NonSpace, Word},
};
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

type Graph = HashMap<String, HashSet<String>>;

fn all_paths(from: &str, to: &str, graph: &Graph) -> usize {
    if from == to {
        return 1;
    }
    graph
        .get(from)
        .map(|edges| edges.iter().map(|edge| all_paths(edge, to, graph)).sum())
        .unwrap_or(0)
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut graph: Graph = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line?;
        scan!(&line;
            (let node: Word<String>, ":", [ let edges: NonSpace<String> ]+:HashSet<String>) => {
                graph.insert(node, edges);
            },
        )
        .unwrap();
    }

    let res = all_paths("you", "out", &graph);

    println!("{res}");

    Ok(())
}
