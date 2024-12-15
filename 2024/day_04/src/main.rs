use std::{fs::read_to_string, iter};

use itertools::Itertools;

#[derive(Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn step(self, direction: Direction) -> Option<Self> {
        let (x, y) = match direction {
            Direction::Up => (self.x, self.y.checked_sub(1)?),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x.checked_sub(1)?, self.y),
            Direction::Right => (self.x + 1, self.y),
            Direction::UpLeft => (self.x.checked_sub(1)?, self.y.checked_sub(1)?),
            Direction::UpRight => (self.x + 1, self.y.checked_sub(1)?),
            Direction::DownLeft => (self.x.checked_sub(1)?, self.y + 1),
            Direction::DownRight => (self.x + 1, self.y + 1),
        };

        Some(Position { x, y })
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft,
}

impl Direction {
    fn all() -> [Self; 8] {
        [
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
            Self::UpLeft,
            Self::UpRight,
            Self::DownLeft,
            Self::DownRight,
        ]
    }
}

fn is_xmas(grid: &[Vec<char>], position: Position, direction: Direction) -> bool {
    let width = grid[0].len();
    let height = grid.len();
    let chars = iter::successors(
        Some(position).filter(|p| p.x < width && p.y < height),
        |p| p.step(direction).filter(|p| p.x < width && p.y < height),
    )
    .take(4)
    .flat_map(|p| grid.get(p.y).map(|r| r.get(p.x)))
    .flatten()
    .copied()
    .collect_vec();

    chars == vec!['X', 'M', 'A', 'S']
}

fn count_xmas(grid: &[Vec<char>]) -> usize {
    let width = grid[0].len();
    let height = grid.len();
    (0..width)
        .cartesian_product(0..height)
        .map(|(x, y)| Position { x, y })
        .cartesian_product(Direction::all())
        .filter(|(p, d)| is_xmas(grid, *p, *d))
        .count()
}

fn is_centre_of_x_mas(grid: &[Vec<char>], position: Position) -> bool {
    grid[position.y][position.x] == 'A'
        && ["MMSS", "SSMM", "SMMS", "MSSM"].contains(
            &[
                Direction::UpRight,
                Direction::DownRight,
                Direction::DownLeft,
                Direction::UpLeft,
            ]
            .iter()
            .filter_map(|d| position.step(*d))
            .flat_map(|p| grid.get(p.y).map(|r| r.get(p.x)))
            .flatten()
            .copied()
            .collect::<String>()
            .as_str(),
        )
}

fn count_x_mas(grid: &[Vec<char>]) -> usize {
    let width = grid[0].len();
    let height = grid.len();
    (1..width - 1)
        .cartesian_product(1..height - 1)
        .map(|(x, y)| Position { x, y })
        .filter(|p| is_centre_of_x_mas(grid, *p))
        .count()
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let grid: Vec<_> = input.lines().map(|l| l.chars().collect()).collect();
    let outptu_1 = count_xmas(&grid);
    let output_2 = count_x_mas(&grid);
    (outptu_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
