// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;

#[macro_use]
extern crate scan_rules;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut ranges = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let_scan!(line; (let b: u64, "-", let e: u64));
        ranges.push(b..=e);
    }

    let mut count = 0;
    while let Some(Ok(line)) = lines.next() {
        let id: u64 = line.parse()?;

        if ranges.iter().any(|range| range.contains(&id)) {
            count += 1;
        }
    }
    println!("{count}");

    Ok(())
}
