use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

fn update(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let (l, r) = stone_str.split_at(stone_str.len() / 2);
        return vec![l.parse().unwrap(), r.parse().unwrap()];
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
