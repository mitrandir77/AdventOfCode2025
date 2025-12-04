// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;

fn max_joltage(bank: &[u32]) -> u32 {
    let Some((pos, first_digit)) = bank[..bank.len() - 1]
        .iter()
        .enumerate()
        .max_by(|(pos_a, val_a), (pos_b, val_b)| val_a.cmp(val_b).then(pos_b.cmp(pos_a)))
    else {
        panic!("bank is empty");
    };
    let Some(second_digit) = bank[pos + 1..bank.len()].iter().max() else {
        panic!("bank is empty");
    };
    first_digit * 10 + second_digit
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let digits: Vec<_> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        sum += max_joltage(&digits);
    }
    println!("{sum}");
    Ok(())
}
