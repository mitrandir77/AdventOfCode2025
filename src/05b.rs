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

    ranges.sort_by_key(|range| (*range.start(), *range.end()));

    let mut last = 0;
    let mut sum = 0;
    for range in ranges {
        sum += ((last + 1).max(*range.start())..=*range.end()).count() as u64;
        last = last.max(*range.end());
    }
    println!("{sum}");

    Ok(())
}
