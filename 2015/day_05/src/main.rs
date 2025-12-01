use std::fs::read_to_string;

use itertools::Itertools;

fn is_vowel(c: char) -> bool {
    ['a', 'e', 'i', 'o', 'u'].contains(&c)
}

fn contains_double(s: &str) -> bool {
    s.chars().tuple_windows().any(|(x, y)| x == y)
}

fn does_not_contain_bad_strings(s: &str) -> bool {
    ["ab", "cd", "pq", "xy"].into_iter().all(|x| !s.contains(x))
}

fn is_nice(s: &str) -> bool {
    s.chars().filter(|c| is_vowel(*c)).count() >= 3
        && contains_double(s)
        && does_not_contain_bad_strings(s)
}

fn contains_disjoint_pair(s: &str) -> bool {
    (0..s.len() - 2)
        .map(|i| (&s[i..i + 2], &s[i + 2..]))
        .any(|(x, y)| y.contains(x))
}

fn contains_sandwich(s: &str) -> bool {
    s.chars().tuple_windows().any(|(x, _, y)| x == y)
}

fn is_nice_2(s: &str) -> bool {
    contains_disjoint_pair(s) && contains_sandwich(s)
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let output_1 = input.lines().filter(|s| is_nice(s)).count();
    let output_2 = input.lines().filter(|s| is_nice_2(s)).count();
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
