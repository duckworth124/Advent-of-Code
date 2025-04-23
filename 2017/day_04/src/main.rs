use itertools::Itertools;
use std::fs::read_to_string;

fn solve(input: &str) -> (usize, usize) {
    let output_1 = input
        .lines()
        .filter(|l| l.split_whitespace().all_unique())
        .count();

    let output_2 = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.chars().sorted_unstable().collect_vec())
                .collect_vec()
        })
        .filter(|l| l.iter().all_unique())
        .count();

    (output_1, output_2)
}
fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
