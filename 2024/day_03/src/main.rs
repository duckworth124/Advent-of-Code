use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char, u32},
    multi::{many0, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use std::{fs::read_to_string, time::Instant};

fn parse_mul(input: &str) -> IResult<&str, u32> {
    delimited(tag("mul("), separated_pair(u32, char(','), u32), char(')'))
        .map(|(x, y)| x * y)
        .parse(input)
}

fn mul_sum(input: &str) -> u32 {
    many0(many_till(anychar, parse_mul).map(|(_, i)| i))
        .map(|v| v.into_iter().sum())
        .parse(input)
        .unwrap()
        .1
}

fn mul_sum_cond(input: &str) -> u32 {
    input
        .split("do()")
        .flat_map(|s| s.split("don't()").next())
        .map(mul_sum)
        .sum()
}

fn solve(path: &str) -> (u32, u32) {
    let input = read_to_string(path).unwrap();
    let output_1 = mul_sum(&input);
    let output_2 = mul_sum_cond(&input);
    (output_1, output_2)
}

fn main() {
    let now = Instant::now();
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", now.elapsed().as_secs_f64())
}
