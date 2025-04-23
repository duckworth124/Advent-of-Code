use itertools::Itertools;
use std::fs::read_to_string;

fn solve(input: &str) -> (u32, u32) {
    let numbers = input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let output_1 = numbers
        .iter()
        .map(|v| match v.iter().minmax() {
            itertools::MinMaxResult::NoElements => panic!(),
            itertools::MinMaxResult::OneElement(_) => 0,
            itertools::MinMaxResult::MinMax(x, y) => y - x,
        })
        .sum();

    let output_2 = numbers
        .iter()
        .map(|v| {
            v.iter()
                .permutations(2)
                .map(|v| (v[0], v[1]))
                .map(|(x, y)| (x.min(y), x.max(y)))
                .find(|&(x, y)| y % x == 0)
                .map(|(x, y)| y / x)
                .unwrap()
        })
        .sum();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
