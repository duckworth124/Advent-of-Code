use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{anychar, u32};
use nom::{IResult, Parser};
use std::char;
use std::fs::read_to_string;

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

fn solve(path: &str) -> usize {
    let input = read_to_string(path).unwrap();
    input
        .lines()
        .map(process_line)
        .map(|x| x.unwrap().1)
        .map(|(lower, upper, c, s)| (lower, upper, s.chars().filter(|x| *x == c).count()))
        .filter(|(lower, upper, count)| (lower..=upper).contains(&count))
        .count()
}

fn main() {
    let output = solve("input");
    println!("part 1: {output}")
}
