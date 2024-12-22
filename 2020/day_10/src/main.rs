use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string, iter::once};

fn get_adaptors(input: &str) -> Vec<u32> {
    let mut output = input
        .lines()
        .map(|l| l.parse().unwrap())
        .chain(once(0))
        .sorted()
        .collect_vec();
    output.push(output.last().unwrap() + 3);
    output
}

fn count_differences(numbers: &[u32]) -> usize {
    let differences = numbers
        .iter()
        .tuple_windows()
        .map(|(x, y)| y - x)
        .collect_vec();

    differences.iter().filter(|n| **n == 1).count()
        * differences.iter().filter(|n| **n == 3).count()
}

fn count_arrangements<'a>(
    adaptors: &'a [u32],
    lower: u32,
    upper: u32,
    cache: &mut HashMap<(&'a [u32], u32, u32), usize>,
) -> usize {
    if let Some(output) = cache.get(&(adaptors, lower, upper)) {
        return *output;
    }

    if adaptors.is_empty() {
        return if upper - lower <= 3 { 1 } else { 0 };
    };

    let mut output = count_arrangements(&adaptors[1..], adaptors[0], upper, cache);
    let next = *adaptors.get(1).unwrap_or(&upper);
    if next - lower <= 3 {
        output += count_arrangements(&adaptors[1..], lower, upper, cache)
    }

    cache.insert((adaptors, lower, upper), output);
    output
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let adaptors = get_adaptors(&input);
    let output_1 = count_differences(&adaptors);
    let output_2 = count_arrangements(
        &adaptors[1..adaptors.len() - 1],
        0,
        *adaptors.last().unwrap(),
        &mut HashMap::new(),
    );

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
