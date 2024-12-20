use std::fs::read_to_string;

use itertools::Itertools;

fn count_unique(input: &str) -> usize {
    input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .unique()
        .count()
}

fn count_in_all(input: &str) -> usize {
    ('a'..='z')
        .filter(|c| input.lines().all(|l| l.contains(*c)))
        .count()
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let output_1 = input.split("\n\n").map(count_unique).sum();
    let output_2 = input.split("\n\n").map(count_in_all).sum();
    (output_1, output_2)
}
fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
