use std::{fs::read_to_string, str::FromStr};
use winnow::{
    Parser, Result,
    ascii::dec_uint,
    combinator::{repeat, separated, terminated},
    error::ContextError,
};

struct Node {
    metadata: Vec<u32>,
    children: Vec<Self>,
}

impl Node {
    fn parse_next(input: &mut &str) -> Result<Self> {
        let child_count: usize = dec_uint(input)?;
        ' '.parse_next(input)?;
        let meta_count: usize = dec_uint(input)?;
        ' '.parse_next(input)?;
        let children: Vec<Self> =
            repeat(child_count, terminated(Self::parse_next, ' ')).parse_next(input)?;
        let metadata: Vec<u32> =
            separated(meta_count, dec_uint::<&str, u32, ContextError>, ' ').parse_next(input)?;
        Ok(Self { metadata, children })
    }

    fn total_metadata(&self) -> u32 {
        self.metadata
            .iter()
            .copied()
            .chain(self.children.iter().map(Self::total_metadata))
            .sum()
    }

    fn value(&self) -> u32 {
        if self.children.is_empty() {
            return self.metadata.iter().sum();
        }
        self.metadata
            .iter()
            .copied()
            .filter_map(|x| x.checked_sub(1))
            .filter_map(|i| self.children.get(i as usize))
            .map(Self::value)
            .sum()
    }
}

impl FromStr for Node {
    type Err = ContextError;

    fn from_str(mut s: &str) -> std::result::Result<Self, Self::Err> {
        Self::parse_next(&mut s)
    }
}

fn solve(input: &str) -> (u32, u32) {
    let tree: Node = input.parse().unwrap();
    let output_1 = tree.total_metadata();
    let output_2 = tree.value();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
