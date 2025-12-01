use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string, iter};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn step(self, direction: Direction) -> Option<Self> {
        let (x, y) = match direction {
            Direction::Right => (self.x + 1, self.y),
            Direction::Left => (self.x.checked_sub(1)?, self.y),
            Direction::Down => (self.x, self.y + 1),
            Direction::Up => (self.x, self.y.checked_sub(1)?),
        };

        Some(Self { x, y })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Agent {
    position: Position,
    direction: Direction,
}

impl Agent {
    fn step(self, grid: &[Vec<bool>]) -> Option<Self> {
        let width = grid[0].len();
        let height = grid.len();
        let position = self
            .position
            .step(self.direction)
            .filter(|p| p.x < width && p.y < height)?;

        Some(Self { position, ..self })
    }

    fn advance(self, grid: &[Vec<bool>]) -> Option<Self> {
        let next = self.step(grid)?;
        if grid[next.position.y][next.position.x] {
            return Some(Self {
                direction: self.direction.rotate_right(),
                ..self
            });
        }

        Some(next)
    }

    fn path(self, grid: &[Vec<bool>]) -> impl Iterator<Item = Self> + '_ {
        iter::successors(Some(self), move |a| a.advance(grid))
    }
}

fn is_valid_obstruction(grid: &mut [Vec<bool>], start: Agent, position: Position) -> bool {
    grid[position.y][position.x] = true;
    let output = !start.path(grid).all_unique();
    grid[position.y][position.x] = false;
    output
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let mut grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();

    let start = input
        .lines()
        .enumerate()
        .map(|(i, l)| (i, l.chars()))
        .flat_map(|(i, l)| l.enumerate().map(move |(j, c)| (i, j, c)))
        .map(|(y, x, c)| (Position { x, y }, c))
        .find(|(_, c)| *c == '^')
        .unwrap()
        .0;

    let agent = Agent {
        position: start,
        direction: Direction::Up,
    };

    let visited = agent
        .path(&grid)
        .map(|a| a.position)
        .collect::<HashSet<_>>();

    let output_1 = visited.len();

    let output_2 = visited
        .iter()
        .filter(|p| is_valid_obstruction(&mut grid, agent, **p))
        .count();

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
