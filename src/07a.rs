// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
use std::{collections::BTreeSet, io::BufRead};

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut streams = BTreeSet::new();

    let first_line = lines.next().unwrap()?;
    streams.insert(first_line.find('S').unwrap());
    let width = first_line.len();
    let mut sum = 0;

    while let Some(Ok(line)) = lines.next() {
        let splitters: BTreeSet<usize> = line
            .chars()
            .enumerate()
            .filter_map(|(i, c)| (c == '^').then(|| i))
            .collect();
        let mut to_add = Vec::new();
        let mut to_remove = Vec::new();
        for split in splitters.intersection(&streams) {
            sum += 1;
            if split + 1 < width {
                to_add.push(split + 1);
            }
            if *split > 0 {
                to_add.push(split - 1);
            }
            to_remove.push(*split);
        }
        for stream in to_remove {
            streams.remove(&stream);
        }
        for stream in to_add {
            streams.insert(stream);
        }
    }
    println!("{sum}");

    Ok(())
}
