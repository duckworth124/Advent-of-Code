use std::fs::read_to_string;
use winnow::{
    Parser, Result,
    ascii::{alpha0, dec_int},
    combinator::{alt, delimited, preceded, separated},
};

#[derive(PartialEq, Eq)]
enum JsonValue<'a> {
    Array(Vec<Self>),
    Object(Vec<Self>),
    Number(i32),
    String(&'a str),
}

impl<'a> JsonValue<'a> {
    fn parse(input: &mut &'a str) -> Result<Self> {
        alt((
            Self::parse_array,
            Self::parse_object,
            Self::parse_number,
            Self::parse_string,
        ))
        .parse_next(input)
    }

    fn parse_array(input: &mut &'a str) -> Result<Self> {
        delimited('[', separated(0.., Self::parse, ','), ']')
            .map(Self::Array)
            .parse_next(input)
    }

    fn parse_object(input: &mut &'a str) -> Result<Self> {
        delimited(
            '{',
            separated(0.., preceded((Self::parse_string, ':'), Self::parse), ','),
            '}',
        )
        .map(Self::Object)
        .parse_next(input)
    }

    fn parse_number(input: &mut &str) -> Result<Self> {
        dec_int.map(Self::Number).parse_next(input)
    }

    fn parse_string(input: &mut &'a str) -> Result<Self> {
        delimited('"', alpha0, '"')
            .map(Self::String)
            .parse_next(input)
    }

    fn sum_of_numbers(&self, ignore_red: bool) -> i32 {
        match self {
            Self::Array(array) => array.iter().map(|x| x.sum_of_numbers(ignore_red)).sum(),
            Self::Object(items) => {
                if ignore_red && items.contains(&JsonValue::String("red")) {
                    0
                } else {
                    items.iter().map(|x| x.sum_of_numbers(ignore_red)).sum()
                }
            }
            Self::Number(x) => *x,
            Self::String(_) => 0,
        }
    }
}

fn solve(input: &str) -> (i32, i32) {
    let value = JsonValue::parse.parse(input).unwrap();
    let output_1 = value.sum_of_numbers(false);
    let output_2 = value.sum_of_numbers(true);
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(input.trim());
    println!("part 1: {output_1} part 2: {output_2}")
}
