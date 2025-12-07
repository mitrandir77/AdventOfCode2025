// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
use std::{collections::BTreeMap, io::BufRead};

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut columns = BTreeMap::new();
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        for (key, token) in line.split_whitespace().enumerate() {
            columns
                .entry(key)
                .or_insert(Vec::new())
                .push(token.to_string());
        }
    }

    let mut sum: i64 = 0;
    for tokens in columns.values() {
        let op = tokens.last().unwrap().bytes().next().unwrap();
        let mut res = if op == b'*' { 1 } else { 0 };
        for token in tokens[0..(tokens.len() - 1)].iter() {
            let num: i64 = token.parse()?;
            if op == b'*' {
                res *= num;
            } else {
                res += num;
            }
        }
        sum += res;
    }
    println!("{sum}");

    Ok(())
}
