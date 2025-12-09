// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
#[macro_use]
extern crate scan_rules;
use aoc_2025::char_map::Point;
use itertools::Itertools;
use scan_rules::scan;
use std::io::BufRead;

fn area(p1: Point, p2: Point) -> usize {
    (((p1.x as i64 - p2.x as i64).abs() + 1) * ((p1.y as i64 - p2.y as i64).abs() + 1)) as usize
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut points = Vec::new();
    for line in stdin.lock().lines() {
        let line = line?;
        scan!(&line;
            (let x: usize, ",", let y: usize) => {
                let p = Point {x, y};
                points.push(p);

            },
        )
        .unwrap();
    }

    let max_area = points
        .iter()
        .combinations(2)
        .map(|c| area(*c[0], *c[1]))
        .max()
        .unwrap();

    println!("{max_area}");

    Ok(())
}
