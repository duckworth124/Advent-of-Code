use std::{fs::read_to_string, iter};

use tqdm::Iter;

fn generate_sequence(seed: u128, mult: u128, div: u128) -> impl Iterator<Item = u128> {
    iter::successors(Some(seed), move |&prev| Some((prev * mult) % div)).skip(1)
}

fn compare_count(
    a: impl Iterator<Item = u128>,
    b: impl Iterator<Item = u128>,
    count: usize,
) -> usize {
    a.take(count)
        .tqdm()
        .zip(b)
        .map(|(a, b)| (a % (1 << 16), b % (1 << 16)))
        .filter(|(a, b)| a == b)
        .count()
}

const A_MULT: u128 = 16807;
const B_MULT: u128 = 48271;
const DIV: u128 = 2147483647;

fn main() {
    let input = read_to_string("input").unwrap();
    let a_seed: u128 = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let b_seed: u128 = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let output_1 = compare_count(
        generate_sequence(a_seed, A_MULT, DIV),
        generate_sequence(b_seed, B_MULT, DIV),
        40_000_000,
    );

    let output_2 = compare_count(
        generate_sequence(a_seed, A_MULT, DIV).filter(|a| a % 4 == 0),
        generate_sequence(b_seed, B_MULT, DIV).filter(|b| b % 8 == 0),
        5_000_000,
    );

    println!("part 1: {output_1} part 2: {output_2}")
}
