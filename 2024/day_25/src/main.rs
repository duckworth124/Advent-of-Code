use std::fs::read_to_string;

use itertools::Itertools;

fn column_values(input: &str) -> Vec<usize> {
    (0..input.lines().next().unwrap().len())
        .map(|i| {
            input
                .lines()
                .map(|l| l.chars().nth(i).unwrap())
                .filter(|c| *c == '#')
                .count()
        })
        .collect()
}

fn key_values(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n\n")
        .filter_map(|s| s.strip_suffix("#####"))
        .map(column_values)
        .collect()
}

fn lock_values(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n\n")
        .filter_map(|s| s.strip_prefix("#####\n"))
        .map(column_values)
        .collect()
}

fn solve(path: &str) -> usize {
    let input = read_to_string(path).unwrap();
    let keys = key_values(&input);
    let locks = lock_values(&input);
    keys.iter()
        .cartesian_product(&locks)
        .filter(|(k, l)| k.iter().zip(l.iter()).map(|(x, y)| *x + *y).all(|t| t <= 5))
        .count()
}

fn main() {
    let output = solve("input");
    println!("part 1: {output}")
}
