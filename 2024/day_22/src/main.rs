use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string, iter, time::Instant};

fn next_secret(mut current: u64) -> u64 {
    current ^= current << 6;
    current %= 1 << 24;
    current ^= current >> 5;
    current ^= current << 11;
    current %= 1 << 24;
    current
}

fn secret_sequence(seed: u64) -> impl Iterator<Item = u64> {
    iter::successors(Some(seed), |x| Some(next_secret(*x)))
}

fn price_sequence(seed: u64) -> Vec<i8> {
    secret_sequence(seed)
        .take(2001)
        .map(|n| (n % 10) as i8)
        .collect_vec()
}

fn max_bananas(sequences: &[Vec<i8>]) -> u64 {
    let mut change_map = HashMap::new();
    sequences
        .iter()
        .flat_map(|s| {
            s.iter()
                .tuple_windows()
                .map(|(x, y)| (y - x, y))
                .tuple_windows()
                .map(|((d1, _), (d2, _), (d3, _), (d4, v))| ([d1, d2, d3, d4], v))
                .unique_by(|(c, _)| *c)
                .map(|(c, b)| (c, *b as u64))
        })
        .for_each(|(c, b)| *change_map.entry(c).or_default() += b);
    *change_map.values().max().unwrap()
}

fn solve(path: &str) -> (u64, u64) {
    let input = read_to_string(path).unwrap();
    let seeds: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
    let output_1 = seeds
        .iter()
        .map(|x| secret_sequence(*x).nth(2000).unwrap())
        .sum();

    let prices_sequences = seeds.iter().map(|s| price_sequence(*s)).collect_vec();
    let output_2 = max_bananas(&prices_sequences);

    (output_1, output_2)
}

fn main() {
    let now = Instant::now();
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", now.elapsed().as_secs_f64())
}
