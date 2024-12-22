use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

fn has_sum(numbers: &[u64], target: u64) -> bool {
    let wanted: HashMap<u64, u64> = numbers
        .iter()
        .copied()
        .filter_map(|n| Some((n, target.checked_sub(n)?)))
        .collect();

    wanted
        .values()
        .any(|n| wanted.contains_key(n) && n * 2 != target)
}

fn find_weakness(input: &[u64], preamble_length: usize) -> Option<u64> {
    input
        .windows(preamble_length + 1)
        .find(|v| !has_sum(&v[..v.len()], *v.last().unwrap()))
        .map(|v| v.last().unwrap())
        .copied()
}

fn find_contiguous_sum(numbers: &[u64], target: u64) -> &[u64] {
    (0..numbers.len())
        .cartesian_product(0..numbers.len())
        .filter(|(l, r)| l < r)
        .map(|(l, r)| &numbers[l..=r])
        .map(|v| (v, v.iter().sum::<u64>()))
        .find(|(_, s)| *s == target)
        .unwrap()
        .0
}

fn solve(path: &str) -> (u64, u64) {
    let input = read_to_string(path).unwrap();
    let numbers: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let preamble_length = if path == "input" { 25 } else { 5 };
    let output_1 = find_weakness(&numbers, preamble_length).unwrap();
    let v = find_contiguous_sum(&numbers, output_1);
    let output_2 = v.iter().min().unwrap() + v.iter().max().unwrap();
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
