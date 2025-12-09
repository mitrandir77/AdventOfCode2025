// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
#[macro_use]
extern crate scan_rules;
use aoc_2025::char_map::Point;
use itertools::Itertools;
use scan_rules::scan;
use std::{io::BufRead, iter};

fn area(p1: Point, p2: Point) -> usize {
    (((p1.x as i64 - p2.x as i64).abs() + 1) * ((p1.y as i64 - p2.y as i64).abs() + 1)) as usize
}

#[derive(Debug, PartialEq)]
enum Orientation {
    Collinear,
    Clockwise,
    Counterclockwise,
}

type Segment = (Point, Point);

fn orientation(p: &Point, q: &Point, r: &Point) -> Orientation {
    let val = (q.y as i64 - p.y as i64) * (r.x as i64 - q.x as i64)
        - (q.x as i64 - p.x as i64) * (r.y as i64 - q.y as i64);

    match val.cmp(&0) {
        std::cmp::Ordering::Equal => Orientation::Collinear,
        std::cmp::Ordering::Greater => Orientation::Clockwise,
        std::cmp::Ordering::Less => Orientation::Counterclockwise,
    }
}

fn do_intersect(seg1: &Segment, seg2: &Segment) -> bool {
    let (p1, q1) = (&seg1.0, &seg1.1);
    let (p2, q2) = (&seg2.0, &seg2.1);

    let (o1, o2, o3, o4) = (
        orientation(p1, q1, p2),
        orientation(p1, q1, q2),
        orientation(p2, q2, p1),
        orientation(p2, q2, q1),
    );

    o1 != o2 && o3 != o4
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
        .map(|com| {
            // two corners of the rectangle
            let a = Point {
                x: com[1].x.max(com[0].x).checked_sub(1).unwrap_or(0),
                y: com[1].y.max(com[0].y).checked_sub(1).unwrap_or(0),
            };
            let c = Point {
                x: com[1].x.min(com[0].x) + 1,
                y: com[1].y.min(com[0].y) + 1,
            };

            let b = Point { x: a.x, y: c.y };
            let d = Point { x: c.x, y: a.y };

            if points
                .iter()
                .chain(iter::once(&points[0]))
                .tuple_windows()
                .any(|(s1, s2)| {
                    do_intersect(&(*s1, *s2), &(a, b))
                        || do_intersect(&(*s1, *s2), &(b, c))
                        || do_intersect(&(*s1, *s2), &(c, d))
                        || do_intersect(&(*s1, *s2), &(d, a))
                })
            {
                0
            } else {
                area(*com[0], *com[1])
            }
        })
        .max()
        .unwrap();

    println!("{max_area}");

    Ok(())
}
