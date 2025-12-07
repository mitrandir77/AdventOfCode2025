// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut max_len = 9;

    let mut stdin = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let line: Vec<u8> = line.as_bytes().to_vec();
        max_len = max_len.max(line.len());
        stdin.push(line);
    }

    let mut op = b'*';
    let mut res = 0;
    let mut sum = 0;
    for i in 0..max_len {
        let mut num = String::new();

        for line in stdin.iter() {
            if let Some(c) = line.get(i) {
                if c.is_ascii_digit() {
                    num.push(*c as char);
                }
                if *c == b'*' || *c == b'+' {
                    op = *c;
                    res = if op == b'*' { 1 } else { 0 };
                }
            }
        }
        if num.is_empty() {
            sum += res;
            continue;
        }
        let num: i64 = num.parse()?;
        if op == b'*' {
            res *= num;
        } else {
            res += num;
        }
    }
    sum += res;
    println!("{sum}");

    Ok(())
}
