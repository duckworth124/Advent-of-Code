use itertools::Itertools;
use std::fs::read_to_string;
use winnow::{Parser, Result, ascii::alpha0};

#[derive(Debug)]
struct Ipv7 {
    sequences: Vec<String>,
    hypernet_sequences: Vec<String>,
}

impl Ipv7 {
    fn parse(input: &mut &str) -> Result<Self> {
        let mut output = Self {
            sequences: vec![],
            hypernet_sequences: vec![],
        };
        while !input.is_empty() {
            if input.starts_with('[') {
                '['.parse_next(input)?;
                let s = alpha0(input)?;
                ']'.parse_next(input)?;
                output.hypernet_sequences.push(s.to_string());
            } else {
                let s = alpha0(input)?;
                output.sequences.push(s.to_string());
            }
        }

        Ok(output)
    }

    fn is_tls(&self) -> bool {
        if self.hypernet_sequences.iter().any(|s| has_abba(s)) {
            return false;
        }
        self.sequences.iter().any(|s| has_abba(s))
    }

    fn is_ssl(&self) -> bool {
        self.sequences
            .iter()
            .flat_map(|s| s.chars().tuple_windows())
            .filter(|(a, b, c)| a != b && a == c)
            .map(|(a, b, _)| (a, b))
            .any(|(a, b)| {
                self.hypernet_sequences
                    .iter()
                    .any(|sequence| sequence.contains(&format!("{b}{a}{b}")))
            })
    }
}

fn has_abba(mut input: &str) -> bool {
    while input.len() >= 4 {
        let chars: Vec<char> = input.chars().take(4).collect();
        let x = chars[0];
        let y = chars[1];
        if x != y && x == chars[3] && y == chars[2] {
            return true;
        }
        input = &input[1..];
    }
    false
}

fn solve(input: &str) -> (usize, usize) {
    let ips: Vec<Ipv7> = input
        .lines()
        .map(|mut l| Ipv7::parse(&mut l).unwrap())
        .collect();
    let output_1 = ips.iter().filter(|i| i.is_tls()).count();
    let output_2 = ips.iter().filter(|i| i.is_ssl()).count();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
