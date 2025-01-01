use itertools::Itertools;
use std::{fs::read_to_string, iter, time::Instant};

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
    let mut change_map = [0; 19 * 19 * 19 * 19];
    sequences
        .iter()
        .flat_map(|s| {
            s.iter()
                .tuple_windows()
                .map(|(x, y)| (y - x, y))
                .tuple_windows()
                .map(|((d1, _), (d2, _), (d3, _), (d4, v))| ([d1, d2, d3, d4], v))
                .map(|(c, b)| (c, *b as u64))
                .map(|(c, b)| (c.map(|x| x + 9), b))
                .map(|(c, b)| (c.into_iter().fold(0, |acc, x| acc * 19 + x as usize), b))
                .unique_by(|(c, _)| *c)
        })
        .fold(0_u64, |m, (c, b)| {
            change_map[c] += b;
            m.max(change_map[c])
        })
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
