use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::{Add, Neg, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(self, direction: Direction) -> Self {
        self + direction.into()
    }
}

impl From<Direction> for Position {
    fn from(value: Direction) -> Self {
        let (x, y) = match value {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        Self { x, y }
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (x, y) = (self.x + rhs.x, self.y + rhs.y);
        Self { x, y }
    }
}

impl Neg for Position {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let (x, y) = (-self.x, -self.y);
        Self { x, y }
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn process_input(input: &str) -> HashMap<u32, HashSet<Position>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| row.char_indices().map(move |(x, c)| (x, y, c)))
        .map(|(x, y, c)| (x as i32, y as i32, c.to_digit(10).unwrap()))
        .map(|(x, y, h)| (h, Position { x, y }))
        .sorted_unstable_by_key(|x| x.0)
        .chunk_by(|x| x.0)
        .into_iter()
        .map(|(x, g)| (x, g.into_iter().map(|x| x.1).collect()))
        .collect()
}

fn total_scores(positions: &HashMap<u32, HashSet<Position>>) -> usize {
    let mut reachable_trailheads: HashMap<Position, HashSet<Position>> = positions[&0]
        .iter()
        .map(|&p| (p, std::iter::once(p).collect()))
        .collect();

    for height in 1..=9 {
        reachable_trailheads = positions[&height]
            .iter()
            .map(|&p| {
                (
                    p,
                    [
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ]
                    .into_iter()
                    .map(|d| p.step(d))
                    .filter(|p| positions[&(height - 1)].contains(p))
                    .flat_map(|p| &reachable_trailheads[&p])
                    .copied()
                    .collect(),
                )
            })
            .collect();
    }

    reachable_trailheads.values().map(|s| s.len()).sum()
}

fn total_indices(positions: &HashMap<u32, HashSet<Position>>) -> usize {
    let mut number_of_paths: HashMap<Position, usize> =
        positions[&0].iter().map(|&p| (p, 1)).collect();

    for height in 1..=9 {
        number_of_paths = positions[&height]
            .iter()
            .map(|&p| {
                (
                    p,
                    [
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ]
                    .into_iter()
                    .map(|d| p.step(d))
                    .filter(|p| positions[&(height - 1)].contains(p))
                    .map(|p| &number_of_paths[&p])
                    .sum(),
                )
            })
            .collect();
    }

    number_of_paths.values().sum()
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let positions = process_input(&input);
    let output_1 = total_scores(&positions);
    let output_2 = total_indices(&positions);
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
