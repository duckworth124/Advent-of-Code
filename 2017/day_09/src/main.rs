use std::fs::read_to_string;

use regex::Regex;
use winnow::{
    Parser, Result,
    combinator::{alt, not, repeat, separated},
    token::none_of,
};

#[derive(Clone, Debug)]
enum Object {
    Garbage(String),
    Group(Vec<Object>),
}

impl Object {
    fn parse_group(input: &mut &str) -> Result<Self> {
        '{'.parse_next(input)?;
        let inner: Vec<Self> =
            separated(0.., alt((Self::parse_garbage, Self::parse_group)), ',').parse_next(input)?;
        '}'.parse_next(input)?;
        Ok(Self::Group(inner))
    }

    fn parse_garbage(input: &mut &str) -> Result<Self> {
        '<'.parse_next(input)?;
        let inner = repeat(0.., none_of('>')).parse_next(input)?;
        '>'.parse_next(input)?;
        Ok(Self::Garbage(inner))
    }

    fn total_score(&self, base: u32) -> u32 {
        match self {
            Self::Garbage(_) => 0,
            Self::Group(inner) => base + inner.iter().map(|g| g.total_score(base + 1)).sum::<u32>(),
        }
    }

    fn total_garbage_len(&self) -> usize {
        match self {
            Self::Garbage(s) => s.len(),
            Self::Group(inner) => inner.iter().map(|o| o.total_garbage_len()).sum(),
        }
    }
}

fn remove_cancelled(input: &str) -> String {
    let pat = Regex::new("!.").unwrap();
    pat.replace_all(input, "").to_string()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let input = remove_cancelled(&input);
    let object = Object::parse_group(&mut input.trim()).unwrap();
    let output_1 = object.total_score(1);
    let output_2 = object.total_garbage_len();
    println!("part 1: {output_1} part 2: {output_2}")
}
