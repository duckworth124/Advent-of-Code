use std::{collections::HashMap, fs::read_to_string};

fn solve(input: &str) -> (String, String) {
    let len = input.lines().next().unwrap().len();
    let mut counts: Vec<HashMap<char, u32>> = vec![HashMap::new(); len];
    for line in input.lines() {
        for (i, c) in line.char_indices() {
            *counts[i].entry(c).or_default() += 1;
        }
    }
    let output_1 = counts
        .iter()
        .map(|m| m.iter().max_by_key(|(_, n)| **n).unwrap().0)
        .collect();

    let output_2 = counts
        .iter()
        .map(|m| m.iter().min_by_key(|(_, n)| **n).unwrap().0)
        .collect();
    (output_1, output_2)
}
fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
