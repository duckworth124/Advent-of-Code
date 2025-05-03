use std::fs::read_to_string;

use winnow::{
    Parser, Result,
    ascii::dec_uint,
    combinator::{delimited, separated_pair},
};

fn decrompressed_len(mut input: &str) -> usize {
    let mut output = 0;
    while !input.is_empty() {
        if let Ok((len, count)) = parse_marker(&mut input) {
            output += len * count;
            input = &input[len..];
            continue;
        }
        output += 1;
        input = &input[1..];
    }

    output
}

fn decrompressed_len_2(mut input: &str) -> usize {
    let mut output = 0;
    while !input.is_empty() {
        if let Ok((len, count)) = parse_marker(&mut input) {
            let s = &input[..len];
            output += decrompressed_len_2(s) * count;
            input = &input[len..];
            continue;
        }
        output += 1;
        input = &input[1..];
    }

    output
}

fn parse_marker(input: &mut &str) -> Result<(usize, usize)> {
    delimited('(', separated_pair(dec_uint, 'x', dec_uint), ')').parse_next(input)
}

fn solve(input: &str) -> (usize, usize) {
    let output_1 = decrompressed_len(input.trim());
    let output_2 = decrompressed_len_2(input.trim());
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
