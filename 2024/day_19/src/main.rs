use std::{collections::HashMap, fs::read_to_string};

fn count_possible<'a>(
    target: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&output) = cache.get(target) {
        return output;
    }

    if target.is_empty() {
        cache.insert(target, 1);
        return 1;
    }

    let output = patterns
        .iter()
        .flat_map(|s| target.strip_prefix(s))
        .map(|s| count_possible(s, patterns, cache))
        .sum();

    cache.insert(target, output);
    output
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
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
        .map(|t| count_possible(t, &patterns, &mut HashMap::new()))
        .collect();

    let output_1 = counts.iter().filter(|&&n| n > 0).count();
    let output_2 = counts.iter().sum();
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
