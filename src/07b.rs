// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
use memoized::{Memoized, memoize_rec};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::BufRead,
};

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut streams = BTreeSet::new();

    let first_line = lines.next().unwrap()?;
    let start_pos = first_line.find('S').unwrap();
    streams.insert(start_pos);
    let width = first_line.len();
    let mut t = 0;

    let mut splits = BTreeMap::new();

    for line in lines {
        let line = line?;
        t += 1;
        let splitters: BTreeSet<usize> = line
            .chars()
            .enumerate()
            .filter_map(|(i, c)| (c == '^').then(|| i))
            .collect();
        let mut to_add = Vec::new();
        let mut to_remove = Vec::new();
        for split in splitters.intersection(&streams) {
            let mut to_add_split = Vec::new();
            if split + 1 < width {
                to_add_split.push(split + 1);
            }
            if *split > 0 {
                to_add_split.push(split - 1);
            }
            to_add.extend(to_add_split.iter().copied());
            splits.insert((t, *split), to_add_split);
            to_remove.push(*split);
        }
        for stream in to_remove {
            streams.remove(&stream);
        }
        for stream in to_add {
            streams.insert(stream);
        }
    }

    let end_of_time = t + 1;

    let mut timelines = memoize_rec(|timelines, (t, pos): (usize, usize)| {
        if t >= end_of_time {
            return 1 as usize;
        }

        if let Some(splits) = splits.get(&(t, pos)) {
            let mut ret = 0;
            for split in splits {
                ret += timelines((t + 1, *split));
            }
            return ret;
        }
        timelines((t + 1, pos))
    });

    let timelines_count = timelines.call((0, start_pos));
    println!("{timelines_count}");

    Ok(())
}
