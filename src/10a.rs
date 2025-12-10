// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
use itertools::Itertools;
use std::{io::BufRead, usize};

fn str_to_bitmask(s: &str) -> usize {
    assert!(s.len() <= 64);
    let mut sum = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '#' {
            sum += 1 << i;
        }
    }
    sum
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();

    let mut sum = 0;
    while let Some(Ok(line)) = lines.next() {
        let groups: Vec<_> = line.split(" ").collect();
        let goal = str_to_bitmask(&groups[0][1..groups[0].len() - 1]);
        let _joltage: Vec<usize> = groups[groups.len() - 1][1..groups[groups.len() - 1].len() - 1]
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let mut buttons: Vec<usize> = Vec::new();
        for group in &groups[1..groups.len() - 1] {
            buttons.push(
                group[1..group.len() - 1]
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .fold(0_usize, |sum, v: usize| sum + (1 << v)),
            );
        }

        'outer: for l in 0..buttons.len() {
            for perm in buttons.iter().combinations(l) {
                let mut current = 0;
                for button in perm.iter() {
                    current ^= **button;
                    if current == goal {
                        sum += l;
                        break 'outer;
                    }
                }
            }
        }
    }
    println!("{sum}");

    Ok(())
}
