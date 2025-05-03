use std::fs::read_to_string;

use itertools::Itertools;

fn is_triangle(sides: &mut [u32]) -> bool {
    sides.sort_unstable();
    sides[0] + sides[1] > sides[2]
}

fn solve(input: &str) -> (usize, usize) {
    let nums: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let output_1 = nums
        .iter()
        .cloned()
        .map(|mut v| is_triangle(&mut v))
        .filter(|b| *b)
        .count();

    let output_2 = (0..3)
        .flat_map(|i| nums.iter().map(move |v| v[i]))
        .chunks(3)
        .into_iter()
        .map(|v| is_triangle(&mut v.collect_vec()))
        .filter(|b| *b)
        .count();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
