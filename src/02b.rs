use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::io;
use std::io::Read;

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut sum: i64 = 0;

    for (_, [a, b]) in re.captures_iter(&buffer).map(|c| c.extract()) {
        let (a, b): (i64, i64) = (a.parse()?, b.parse()?);

        for i in a..=b {
            let s: Vec<char> = i.to_string().chars().collect();
            let l = s.len();
            'outer: for chunk_size in 1..=l / 2 {
                if l % chunk_size != 0 {
                    continue;
                }
                let mut different = false;
                for (x, y) in s.chunks(chunk_size).tuple_windows() {
                    if x != y {
                        different = true;
                        break;
                    }
                }
                if !different {
                    sum += i;
                    // dbg!(i);
                    break 'outer;
                }
            }
        }
    }
    println!("{sum}");
    Ok(())
}
