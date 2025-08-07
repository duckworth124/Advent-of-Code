use std::fs::read_to_string;

use winnow::{
    Parser, Result,
    ascii::dec_uint,
    combinator::{alt, delimited, repeat, separated, separated_pair},
};

fn eval_expr(input: &mut &str) -> Result<u64> {
    let init = eval_term(input)?;
    repeat(0.., (alt((" + ", " * ")), eval_term))
        .fold(
            || init,
            |acc, (op, x)| if op == " + " { acc + x } else { acc * x },
        )
        .parse_next(input)
}

fn eval_term(input: &mut &str) -> Result<u64> {
    alt((dec_uint, eval_bracketed)).parse_next(input)
}

fn eval_bracketed(input: &mut &str) -> Result<u64> {
    delimited('(', eval_expr, ')').parse_next(input)
}

fn eval_expr_2(input: &mut &str) -> Result<u64> {
    separated(0.., eval_term_2, " * ")
        .map(|v: Vec<u64>| v.into_iter().product())
        .parse_next(input)
}

fn eval_term_2(input: &mut &str) -> Result<u64> {
    separated(
        0..,
        alt((dec_uint, delimited('(', eval_expr_2, ')'))),
        " + ",
    )
    .map(|v: Vec<u64>| v.into_iter().sum())
    .parse_next(input)
}

fn solve(input: &str) -> (u64, u64) {
    let output_1 = input.lines().map(|l| eval_expr.parse(l).unwrap()).sum();
    let output_2 = input.lines().map(|l| eval_expr_2.parse(l).unwrap()).sum();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}");
}
