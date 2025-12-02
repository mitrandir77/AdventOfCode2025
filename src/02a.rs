use anyhow::Result;
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
            let s = i.to_string();
            let l = s.len();
            if (l % 2 == 0) && s[0..l / 2] == s[l / 2..l] {
                // dbg!(i);
                sum += i;
            }
        }
    }
    println!("{sum}");
    Ok(())
}
