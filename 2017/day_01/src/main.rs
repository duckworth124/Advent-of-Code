use itertools::Itertools;
use std::fs::read_to_string;

fn solve(input: &str) -> (u32, u32) {
    let digits = input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let output_1 = digits
        .iter()
        .map(|l| {
            l.iter()
                .copied()
                .chain([l[0]])
                .tuple_windows()
                .filter(|(x, y)| x == y)
                .map(|(x, _)| x)
                .sum::<u32>()
        })
        .sum();

    let output_2 = digits
        .iter()
        .map(|l| {
            let (left, right) = l.split_at(l.len() / 2);
            left.iter()
                .zip(right)
                .filter(|(x, y)| x == y)
                .map(|(x, _)| x)
                .sum::<u32>()
        })
        .sum::<u32>()
        * 2;

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
