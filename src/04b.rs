// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
use aoc_2025::char_map::Map;
use itertools::Itertools;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut map = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        map.push(line.as_bytes().iter().cloned().collect_vec());
    }
    let mut map = Map::new(map);
    let mut sum = 0;

    loop {
        let mut accessible = Vec::new();
        for (p, val) in map.points() {
            if val == b'@' {
                if map
                    .neighbours(p)
                    .into_iter()
                    .filter_map(|n| map.get(n))
                    .filter(|val| *val == b'@')
                    .count()
                    < 4
                {
                    accessible.push(p);
                }
            }
        }
        if accessible.is_empty() {
            break;
        }
        sum += accessible.len();
        for p in accessible.into_iter() {
            map.set(p, b'x');
        }
    }
    println!("{sum}");

    Ok(())
}
