use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

fn prime_factor_decomposition(mut n: u32) -> HashMap<u32, u32> {
    let mut output = HashMap::new();
    let mut p = 2;
    while n > 1 {
        if n % p == 0 {
            n /= p;
            *output.entry(p).or_default() += 1;
            continue;
        }
        if p * p > n {
            p = n;
            continue;
        }
        p += 1
    }

    output
}

fn all_factors(n: u32) -> Vec<u32> {
    prime_factor_decomposition(n)
        .into_iter()
        .map(|(p, n)| (0..=n).map(move |i| p.pow(i)))
        .multi_cartesian_product()
        .map(|v| v.into_iter().product())
        .collect()
}

fn num_presents(house: u32) -> u32 {
    all_factors(house).into_iter().sum::<u32>() * 10
}

fn num_presents_2(house: u32) -> u32 {
    all_factors(house)
        .into_iter()
        .filter(|&p| house / p <= 50)
        .sum::<u32>()
        * 11
}

fn solve(input: &str) -> (u32, u32) {
    let target: u32 = input.trim().parse().unwrap();

    let output_1 = (1..)
        .filter(|x| x * (x - 1) * 10 / 2 >= target)
        .inspect(|x| println!("{x} {} {}", num_presents(*x), target))
        .find(|house| num_presents(*house) >= target)
        .unwrap();

    let output_2 = (1..)
        .filter(|x| x * (x - 1) * 10 / 2 >= target)
        .inspect(|x| println!("{x} {} {}", num_presents(*x), target))
        .find(|house| num_presents_2(*house) >= target)
        .unwrap();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
