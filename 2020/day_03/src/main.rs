use itertools::{iterate, Itertools};
use std::{
    fs::read_to_string,
    ops::{Add, Neg, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct Position {
    x: i32,
    y: i32,
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

fn count_trees(grid: &[Vec<bool>], velocity: Position) -> usize {
    iterate(Position::default(), |p| *p + velocity)
        .map(|p| (p.x, p.y))
        .map(|(x, y)| (x as usize, y as usize))
        .take_while(|&(_, y)| y < grid.len())
        .map(|(x, y)| (x % grid[0].len(), y))
        .map(|(x, y)| grid[y][x])
        .filter(|b| *b)
        .count()
}

fn process_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let grid = process_input(&input);
    let output_1 = count_trees(&grid, Position { x: 3, y: 1 });
    let to_check = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(x, y)| Position { x, y })
        .collect_vec();
    let output_2 = to_check
        .into_iter()
        .map(|p| count_trees(&grid, p))
        .product();
    (output_1, output_2)
}
fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
