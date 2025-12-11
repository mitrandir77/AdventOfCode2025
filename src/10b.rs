// Advent of Code 2025
// (c) 2025 Mateusz Kwapich

use anyhow::Result;
use std::{collections::HashMap, io::BufRead};

fn checked_sub_vec(a: &[u16], b: &[u16]) -> Option<Vec<u16>> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).map(|(x, y)| x.checked_sub(*y)).collect()
}
fn add_vec(a: &[u16], b: &[u16]) -> Vec<u16> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).map(|(x, y)| *x + *y).collect()
}

fn solve_internal(
    buttons: &[Vec<u16>],
    target_joltage: Vec<u16>,
    min_buttons: &mut HashMap<Vec<u16>, Option<usize>>,
) -> Option<usize> {
    if let Some(result) = min_buttons.get(&target_joltage) {
        return *result;
    }

    let res = buttons
        .iter()
        .filter_map(|button| checked_sub_vec(&target_joltage, button))
        .filter_map(|cand| solve_internal(buttons, cand, min_buttons))
        .min()
        .map(|res| res + 1);

    min_buttons.insert(target_joltage, res);
    res
}

fn solve(buttons: &Vec<Vec<u16>>, target_joltage: &Vec<u16>) -> usize {
    let n = target_joltage.len();
    let mut min_buttons = HashMap::new();
    let start = vec![0; n];
    let _ = min_buttons.insert(start.clone(), Some(0));
    // heurestics start
    // let max_joltage = *target_joltage.iter().max().unwrap() as usize;
    // for button in buttons {
    //     let mut sum = button.clone();
    //     for i in 1..max_joltage {
    //         min_buttons.insert(sum.clone(), Some(i));
    //         sum = add_vec(&sum, button);
    //     }
    // }
    // heurestics end
    let res = solve_internal(buttons, target_joltage.clone(), &mut min_buttons).unwrap();
    dbg!(min_buttons.len());
    res
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();
    let mut sum = 0;

    while let Some(Ok(line)) = lines.next() {
        let groups: Vec<_> = line.split(" ").collect();
        let _goal = &groups[0][1..groups[0].len() - 1];
        let joltage: Vec<u16> = groups[groups.len() - 1][1..groups[groups.len() - 1].len() - 1]
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let mut buttons: Vec<Vec<u16>> = Vec::new();
        for group in &groups[1..groups.len() - 1] {
            let mut button = vec![0; joltage.len()];
            for n in group[1..group.len() - 1]
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
            {
                button[n] = 1;
            }
            buttons.push(button);
        }
        sum += solve(&buttons, &joltage);
    }
    println!("{sum}");

    Ok(())
}
