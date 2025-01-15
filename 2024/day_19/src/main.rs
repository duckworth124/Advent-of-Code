use std::{collections::HashMap, fs::read_to_string};

fn count_possible<'a>(
    target: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&output) = cache.get(target) {
        return output;
    }

    let output = if target.is_empty() {
        1
    } else {
        patterns
            .iter()
            .filter_map(|s| target.strip_prefix(s))
            .map(|s| count_possible(s, patterns, cache))
            .sum()
    };

    cache.insert(target, output);
    output
}

fn count_possible_iterative(target: &str, patterns: &[&str]) -> usize {
    let mut counts = vec![0; target.len() + 1];
    counts[0] = 1;
    for i in 0..target.len() {
        let remaining = &target[i..];
        for pattern in patterns {
            if remaining.starts_with(pattern) {
                counts[i + pattern.len()] += counts[i];
            }
        }
    }
    counts[target.len()]
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let mut cache = HashMap::new();
    let patterns: Vec<&str> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim())
        .collect();
    let targets: Vec<&str> = input.lines().skip(2).collect();
    let counts: Vec<usize> = targets
        .iter()
        .map(|t| count_possible(t, &patterns, &mut cache))
        .collect();

    let output_1 = counts.iter().filter(|&&n| n > 0).count();
    let output_2 = counts.iter().sum();
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
