use std::{fs::read_to_string, ops::Add};

#[derive(Clone, Copy, Debug, Default)]
struct HexPosition {
    n: i32,
    ne: i32,
}

impl Add for HexPosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            n: self.n + rhs.n,
            ne: self.ne + rhs.ne,
        }
    }
}

impl HexPosition {
    fn parse(input: &str) -> Self {
        let (n, ne) = match input {
            "n" => (1, 0),
            "s" => (-1, 0),
            "ne" => (0, 1),
            "sw" => (0, -1),
            "nw" => (1, -1),
            "se" => (-1, 1),
            _ => panic!("unrecognized input: {input}"),
        };
        Self { n, ne }
    }

    fn distance_to_origin(self) -> i32 {
        if self.n * self.ne >= 0 {
            self.n.abs() + self.ne.abs()
        } else {
            self.n.abs().max(self.ne.abs())
        }
    }
}
fn main() {
    let input = read_to_string("input").unwrap();
    let positions: Vec<HexPosition> = input
        .trim()
        .split(',')
        .scan(HexPosition::default(), |acc, x| {
            let output = Some(*acc);
            *acc = *acc + HexPosition::parse(x);
            output
        })
        .collect();
    let output_1 = positions.last().unwrap().distance_to_origin();
    let output_2 = positions
        .iter()
        .map(|p| p.distance_to_origin())
        .max()
        .unwrap();
    println!("part 1: {output_1} part 2: {output_2}")
}
