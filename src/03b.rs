// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;

fn max_joltage(bank: &[u64], battery_count: usize) -> u64 {
    if battery_count == 1 {
        let Some(digit) = bank[0..bank.len()].iter().max() else {
            panic!("bank is empty");
        };
        return *digit;
    }

    let Some((pos, first_digit)) = bank[..bank.len() - battery_count + 1]
        .iter()
        .enumerate()
        .max_by(|(pos_a, val_a), (pos_b, val_b)| val_a.cmp(val_b).then(pos_b.cmp(pos_a)))
    else {
        panic!("bank is empty");
    };
    let recursive_joltage = max_joltage(&bank[pos + 1..bank.len()], battery_count - 1);
    first_digit * 10_u64.pow(battery_count as u32 - 1) + recursive_joltage
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let digits: Vec<_> = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u64)
            .collect();
        sum += max_joltage(&digits, 12);
    }
    println!("{sum}");
    Ok(())
}
