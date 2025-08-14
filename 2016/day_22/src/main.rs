use itertools::Itertools;
use std::{fmt::Display, fs::read_to_string};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    used: u32,
    avail: u32,
}

impl Node {
    fn parse(input: &str) -> Self {
        let nums: Vec<u32> = input
            .split_whitespace()
            .skip(2)
            .take(2)
            .map(|s| &s[..s.len() - 1])
            .map(|s| s.parse().unwrap())
            .collect();
        let used = nums[0];
        let avail = nums[1];
        Self { used, avail }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Grid {
    nodes: Vec<Vec<Node>>,
    data_position: (usize, usize),
}

impl Grid {
    fn parse(input: &str) -> Self {
        Self {
            nodes: input
                .lines()
                .skip(2)
                .map(Node::parse)
                .chunks(25)
                .into_iter()
                .map(|v| v.collect())
                .collect(),
            data_position: (0, 34),
        }
    }

    fn viable_pairs(&self) -> usize {
        self.nodes
            .iter()
            .flatten()
            .permutations(2)
            .filter(|v| v[0].used != 0 && v[0].used <= v[1].avail)
            .count()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.nodes {
            for n in row {
                if n.used == 0 {
                    write!(f, "_")?
                } else if n.used > 100 {
                    write!(f, "#")?
                } else {
                    write!(f, ".")?
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve(input: &str) -> (usize, u32) {
    let grid = Grid::parse(input);
    println!(
        "{:?}",
        grid.nodes.iter().flatten().map(|n| n.used).k_smallest(3)
    );
    let output_1 = grid.viable_pairs();
    let output_2 = 35 + 33 * 5;
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2 (worked out by looking at the grid): {output_2}")
}
