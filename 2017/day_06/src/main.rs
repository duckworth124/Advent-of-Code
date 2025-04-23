use std::{cmp::Reverse, collections::HashMap, fs::read_to_string};

fn redistribute(banks: &mut [u32]) {
    //min prioritieses earlier elements in ties
    let mut index = (0..banks.len()).min_by_key(|&i| Reverse(banks[i])).unwrap();

    let mut blocks = banks[index];
    banks[index] = 0;
    while blocks > 0 {
        index += 1;
        index %= banks.len();
        banks[index] += 1;
        blocks -= 1;
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut banks: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut visited = HashMap::new();
    let mut steps = 0;

    let (output_1, output_2) = loop {
        match visited.insert(banks.clone(), steps) {
            None => {}
            Some(x) => break (visited.len(), steps - x),
        };
        steps += 1;
        redistribute(&mut banks);
    };

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
