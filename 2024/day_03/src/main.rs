use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, u32},
    combinator::value,
    multi::many0,
    sequence::tuple,
    Parser,
};
use std::fs::read_to_string;

fn mul_sum(input: &str) -> u32 {
    many0(alt((
        tuple((tag("mul("), u32::<&str, ()>, tag(","), u32, tag(")")))
            .map(|(_, x, _, y, _)| x * y)
            .map(Some),
        value(None, anychar),
    )))
    .map(|v| v.into_iter().flatten().sum())
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
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}");
}
