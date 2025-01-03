use std::fs::read_to_string;

use fancy_regex::Regex;

fn is_vowel(c: char) -> bool {
    ['a', 'e', 'i', 'o', 'u'].contains(&c)
}

fn contains_double(s: &str) -> bool {
    let pat = Regex::new(r"(.)\1").unwrap();
    pat.is_match(s).unwrap()
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
    let pat = Regex::new(r"(..).*\1").unwrap();
    pat.is_match(s).unwrap()
}

fn contains_sandwich(s: &str) -> bool {
    let pat = Regex::new(r"(.).\1").unwrap();
    pat.is_match(s).unwrap()
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
