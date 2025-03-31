use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
    let input = read_to_string("input").unwrap();
    let depths: Vec<u32> = input.lines().map(|s| s.parse().unwrap()).collect();
    let output_1 = depths.windows(2).filter(|v| v[1] > v[0]).count();
    let output_2 = depths
        .windows(3)
        .map(|v| v.iter().sum::<u32>())
        .tuple_windows()
        .filter(|(x, y)| y > x)
        .count();

    println!("part 1: {output_1} part 2: {output_2}")
}
