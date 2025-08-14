use itertools::Itertools;
use std::{collections::VecDeque, fs::read_to_string};

fn solve(input: &str) -> (u32, u32) {
    let mut blacklist: VecDeque<(u32, u32)> = input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once('-').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .sorted_unstable()
        .collect();

    let mut output_1 = 0;
    for &(low, high) in &blacklist {
        if output_1 >= low && output_1 <= high {
            output_1 = high + 1;
            continue;
        }
        if output_1 < low {
            break;
        }
    }

    let mut compressed_blacklist = vec![];
    while blacklist.len() > 1 {
        let (low_1, high_1) = blacklist[0];
        let (low_2, high_2) = blacklist[1];
        if high_1 < low_2 {
            compressed_blacklist.push(blacklist.pop_front().unwrap());
            continue;
        }

        let high = high_1.max(high_2);
        blacklist.pop_front();
        blacklist.pop_front();
        blacklist.push_front((low_1, high));
    }
    compressed_blacklist.push(blacklist[0]);
    let output_2 = compressed_blacklist
        .iter()
        .copied()
        .tuple_windows()
        .map(|(r1, r2)| (r1.1 + 1, (r2.0 - 1)))
        .map(|(low, high)| high.saturating_sub(low - 1))
        .sum();

    (output_1, output_2)
}
fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
