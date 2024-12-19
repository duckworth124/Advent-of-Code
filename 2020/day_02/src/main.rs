use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{anychar, u32};
use nom::{IResult, Parser};
use std::char;
use std::fs::read_to_string;
use std::ops::RangeBounds;

fn process_line(line: &str) -> IResult<&str, (usize, usize, char, &str)> {
    u32.and(tag("-"))
        .map(|x| x.0)
        .and(u32)
        .and(tag(" "))
        .map(|x| x.0)
        .and(anychar)
        .map(|((lower, upper), c)| (lower, upper, c))
        .and(tag(": "))
        .map(|x| x.0)
        .and(is_not(""))
        .map(|((lower, upper, c), s)| (lower as usize, upper as usize, c, s))
        .parse(line)
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
    let processed: Vec<_> = input
        .lines()
        .map(process_line)
        .map(|r| r.unwrap().1)
        .collect();
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
