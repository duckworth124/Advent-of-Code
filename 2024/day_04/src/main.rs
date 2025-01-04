use itertools::{iterate, Itertools};
use std::{char, fs::read_to_string, time::Instant};

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(self, direction: Direction) -> Self {
        let (x, y) = match direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
            Direction::UpLeft => (self.x - 1, self.y - 1),
            Direction::UpRight => (self.x + 1, self.y - 1),
            Direction::DownLeft => (self.x - 1, self.y + 1),
            Direction::DownRight => (self.x + 1, self.y + 1),
        };

        Position { x, y }
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

fn get(grid: &[Vec<char>], position: Position) -> Option<char> {
    let y: usize = position.y.try_into().ok()?;
    let x: usize = position.x.try_into().ok()?;
    grid.get(y)?.get(x).copied()
}

fn is_xmas(grid: &[Vec<char>], position: Position, direction: Direction) -> bool {
    let chars = iterate(position, |p| p.step(direction))
        .take(4)
        .flat_map(|p| get(grid, p))
        .collect_vec();

    chars == vec!['X', 'M', 'A', 'S']
}

fn count_xmas(grid: &[Vec<char>]) -> usize {
    let width = grid[0].len();
    let height = grid.len();
    (0..width)
        .cartesian_product(0..height)
        .map(|(x, y)| (x as i32, y as i32))
        .map(|(x, y)| Position { x, y })
        .cartesian_product(Direction::all())
        .filter(|(p, d)| is_xmas(grid, *p, *d))
        .count()
}

fn is_centre_of_x_mas(grid: &[Vec<char>], position: Position) -> bool {
    grid[position.y as usize][position.x as usize] == 'A'
        && ["MMSS", "SSMM", "SMMS", "MSSM"].contains(
            &[
                Direction::UpRight,
                Direction::DownRight,
                Direction::DownLeft,
                Direction::UpLeft,
            ]
            .iter()
            .map(|d| position.step(*d))
            .flat_map(|p| get(grid, p))
            .collect::<String>()
            .as_str(),
        )
}

fn count_x_mas(grid: &[Vec<char>]) -> usize {
    let width = grid[0].len();
    let height = grid.len();
    (1..width - 1)
        .cartesian_product(1..height - 1)
        .map(|(x, y)| (x as i32, y as i32))
        .map(|(x, y)| Position { x, y })
        .filter(|p| is_centre_of_x_mas(grid, *p))
        .count()
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let grid: Vec<_> = input.lines().map(|l| l.chars().collect()).collect();
    let output_1 = count_xmas(&grid);
    let output_2 = count_x_mas(&grid);
    (output_1, output_2)
}

fn main() {
    let now = Instant::now();
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", now.elapsed().as_secs_f64())
}
