// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
#[macro_use]
extern crate scan_rules;
use itertools::Itertools;
use scan_rules::scan;
use std::io::BufRead;

use union_find_rs::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut points = Vec::new();
    let mut circuits = DisjointSets::new();
    for line in stdin.lock().lines() {
        let line = line?;
        scan!(&line;
            (let x: i64, ",", let y:i64, ",",  let z: i64) => {
                let p = Point {x, y, z};
                points.push(p);
                circuits.make_set(p).unwrap();

            },
        )
        .unwrap();
    }
    let mut pairs: Vec<(Point, Point)> = points
        .iter()
        .combinations(2)
        .map(|c| (*c[0], *c[1]))
        .collect();

    pairs.sort_by_key(|(p1, p2)| p1.distance_squared(p2));

    let mut last = None;
    for (p1, p2) in pairs.iter() {
        if circuits.find_set(p1).unwrap() != circuits.find_set(p2).unwrap() {
            circuits.union(&p1, &p2).unwrap();
            last = Some((*p1, *p2));
        }
    }

    let (p1, p2) = last.unwrap();
    println!("{}", p1.x * p2.x);

    Ok(())
}
