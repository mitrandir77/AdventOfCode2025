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
type Memory = HashMap<(String, String), usize>;

fn all_paths(from: &str, to: &str, graph: &Graph, memory: &mut Memory) -> usize {
    if from == to {
        return 1;
    }
    if let Some(res) = memory.get(&(from.to_string(), to.to_string())) {
        return *res;
    }

    let res = graph
        .get(from)
        .map(|edges| {
            edges
                .iter()
                .map(|edge| all_paths(edge, to, graph, memory))
                .sum()
        })
        .unwrap_or(0);

    memory.insert((from.to_string(), to.to_string()), res);
    res
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

    let mut memory = HashMap::new();
    let dac_fft = all_paths("svr", "dac", &graph, &mut memory)
        * all_paths("dac", "fft", &graph, &mut memory)
        * all_paths("fft", "out", &graph, &mut memory);

    let fft_dac = all_paths("svr", "fft", &graph, &mut memory)
        * all_paths("fft", "dac", &graph, &mut memory)
        * all_paths("dac", "out", &graph, &mut memory);

    let res = fft_dac + dac_fft;

    println!("{res}");

    Ok(())
}
