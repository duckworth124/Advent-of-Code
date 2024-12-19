use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, anychar, u32},
    sequence::tuple,
    Parser,
};
use std::fs::read_to_string;

fn process_line(line: &str) -> (usize, usize, char, &str) {
    tuple((
        u32::<&str, ()>,
        tag("-"),
        u32,
        tag(" "),
        anychar,
        tag(": "),
        alpha0,
    ))
    .map(|(l, _, u, _, c, _, s)| (l as usize, u as usize, c, s))
    .parse(line)
    .unwrap()
    .1
}

fn is_valid_part_1(lower: usize, upper: usize, c: char, s: &str) -> bool {
    let count = s.chars().filter(|&ch| ch == c).count();
    (lower..=upper).contains(&count)
}

fn is_valid_part_2(first: usize, second: usize, c: char, s: &str) -> bool {
    let first = s.chars().nth(first - 1).unwrap();
    let second = s.chars().nth(second - 1).unwrap();
    (first == c) ^ (second == c)
}
fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let processed: Vec<_> = input.lines().map(process_line).collect();
    let output_1 = processed
        .iter()
        .filter(|&&(l, u, c, s)| is_valid_part_1(l, u, c, s))
        .count();

    let output_2 = processed
        .iter()
        .filter(|&&(l, u, c, s)| is_valid_part_2(l, u, c, s))
        .count();

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
