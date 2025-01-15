use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

fn update(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let num_digits = stone.ilog10();
    if num_digits % 2 == 0 {
        let d = 10u64.pow(num_digits);
        let (l, r) = (stone / d, stone % d);
        return vec![l, r];
    }

    vec![stone * 2024]
}

fn update_bag(bag: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut output = HashMap::new();
    for (stone, count) in bag {
        for next_stone in update(stone) {
            *output.entry(next_stone).or_default() += count;
        }
    }
    output
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let mut bag = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .counts();

    for _ in 0..25 {
        bag = update_bag(bag);
    }

    let output_1 = bag.values().sum();

    for _ in 0..50 {
        bag = update_bag(bag);
    }

    let output_2 = bag.values().sum();

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
