use std::{fs::read_to_string, iter};

fn fuel(mass: u32) -> u32 {
    iter::successors(Some(mass), |m| Some((m / 3).saturating_sub(2)))
        .take_while(|&x| x > 0)
        .sum()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let masses: Vec<u32> = input.lines().map(|s| s.parse().unwrap()).collect();

    let output_1: u32 = masses.iter().map(|m| m / 3 - 2).sum();
    let output_2: u32 = masses.iter().copied().map(fuel).sum();

    println!("part 1: {output_1} part 2: {output_2}")
}
