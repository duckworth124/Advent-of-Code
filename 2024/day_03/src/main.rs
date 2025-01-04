use std::{fs::read_to_string, time::Instant};
use winnow::{
    ascii::dec_uint,
    combinator::{delimited, repeat, repeat_till, separated_pair},
    token::any,
    PResult, Parser,
};

fn parse_mul(input: &mut &str) -> PResult<u32> {
    delimited("mul(", separated_pair(dec_uint, ',', dec_uint), ')')
        .map(|(x, y): (u32, u32)| x * y)
        .parse_next(input)
}

fn mul_sum(mut input: &str) -> u32 {
    repeat(
        0..,
        repeat_till(0.., any, parse_mul).map(|(_, i): ((), u32)| i),
    )
    .map(|v: Vec<u32>| v.into_iter().sum::<u32>())
    .parse_next(&mut input)
    .unwrap()
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
