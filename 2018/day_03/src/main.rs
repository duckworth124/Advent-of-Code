use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::BitAnd,
};

use itertools::{Itertools, iproduct};
use winnow::{Parser, Result, ascii::dec_uint, combinator::separated_pair};

#[derive(Clone, Copy, Debug)]
struct Claim {
    top_left: (usize, usize),
    bottom_right: (usize, usize),
}

impl Claim {
    const fn is_empty(self) -> bool {
        let width = self.bottom_right.0.saturating_sub(self.top_left.0);

        let height = self.bottom_right.1.saturating_sub(self.top_left.1);
        width == 0 || height == 0
    }

    fn parse(input: &mut &str) -> Result<Self> {
        '#'.parse_next(input)?;
        let _: usize = dec_uint(input)?;
        " @ ".parse_next(input)?;
        let top_left: (usize, usize) = separated_pair(dec_uint, ',', dec_uint).parse_next(input)?;
        ": ".parse_next(input)?;
        let (width, height): (usize, usize) =
            separated_pair(dec_uint, 'x', dec_uint).parse_next(input)?;
        let bottom_right = (top_left.0 + width, top_left.1 + height);
        Ok(Self {
            top_left,
            bottom_right,
        })
    }

    fn to_points(self) -> Vec<(usize, usize)> {
        iproduct!(
            self.top_left.0..self.bottom_right.0,
            self.top_left.1..self.bottom_right.1
        )
        .collect()
    }
}

impl BitAnd for Claim {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let top_left_x = self.top_left.0.max(rhs.top_left.0);
        let top_left_y = self.top_left.1.max(rhs.top_left.1);
        let bottom_right_x = self.bottom_right.0.min(rhs.bottom_right.0).max(top_left_x);
        let bottom_right_y = self.bottom_right.1.min(rhs.bottom_right.1).max(top_left_y);

        Self {
            top_left: (top_left_x, top_left_y),
            bottom_right: (bottom_right_x, bottom_right_y),
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let claims: Vec<Claim> = input
        .lines()
        .map(|mut l| Claim::parse(&mut l).unwrap())
        .collect();

    let mut candidates: HashSet<usize> = (0..claims.len()).collect();

    let output_1 = claims
        .iter()
        .copied()
        .enumerate()
        .permutations(2)
        .map(|v| (v[0], v[1], v[0].1 & v[1].1))
        .inspect(|((i, _), (j, _), o)| {
            if !o.is_empty() {
                candidates.remove(i);
                candidates.remove(j);
            }
        })
        .map(|(_, _, c)| c)
        .flat_map(|c| c.to_points())
        .unique()
        .count();

    let output_2 = candidates.into_iter().next().unwrap() + 1;

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
