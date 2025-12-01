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
    for line in stdin.lock().lines() {
        let line = line?;
        scan!(&line;
            (let dir: char,  let num: i64) => {
                match dir {
                    'L' =>  dial -= num,
                    'R' => dial += num,
                    _ => panic!("invalid direction"),
                }
            },
        )
        .unwrap();
        dial %= 100;
        if dial == 0 {
            zeroes += 1;
        }
    }
    println!("{zeroes}");
    Ok(())
}
