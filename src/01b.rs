// Advent of Code 2024
// (c) 2024 Mateusz Kwapich

use anyhow::Result;
#[macro_use]
extern crate scan_rules;
use scan_rules::scan;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut dial = 50;
    let mut zeroes = 0;
    let mut prev_zero = false;
    for line in stdin.lock().lines() {
        let line = line?;
        let num = scan!(&line;
            (let dir: char,  let num: i64) => {
                match dir {
                    'L' => -num,
                    'R' => num,
                    _ => panic!("invalid direction"),
                }
            },
        )
        .unwrap();

        zeroes += ((num + dial) / 100).abs();
        dial = (num + dial) % 100;

        if dial < 0 {
            dial += 100;
            if !prev_zero {
                zeroes += 1;
            }
        }

        prev_zero = false;
        if dial == 0 {
            if num < 0 {
                zeroes += 1;
            }
            prev_zero = true;
        }
    }
    println!("{zeroes}");
    Ok(())
}
