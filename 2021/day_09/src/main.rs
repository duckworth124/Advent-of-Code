use std::{collections::HashSet, fs::read_to_string};

use itertools::Itertools;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> [Self; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }

    fn step(&self, direction: Direction) -> Option<Self> {
        Some(match direction {
            Direction::Up => Self {
                y: self.y.checked_sub(1)?,
                ..*self
            },

            Direction::Down => Self {
                y: self.y + 1,
                ..*self
            },

            Direction::Left => Self {
                x: self.x.checked_sub(1)?,
                ..*self
            },

            Direction::Right => Self {
                x: self.x + 1,
                ..*self
            },
        })
    }
}

struct Grid(Vec<Vec<u32>>);

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        Grid(grid)
    }

    fn get_height(&self, position: Position) -> Option<u32> {
        self.0.get(position.y)?.get(position.x).copied()
    }

    fn is_low_point(&self, position: Position) -> bool {
        let height = match self.get_height(position) {
            None => return false,
            Some(h) => h,
        };

        Direction::all()
            .into_iter()
            .flat_map(|d| position.step(d))
            .flat_map(|p| self.get_height(p))
            .all(|h| height < h)
    }

    fn low_points(&self) -> Vec<Position> {
        let width = self.0[0].len();
        let height = self.0.len();

        (0..width)
            .cartesian_product(0..height)
            .map(Position::new)
            .filter(|p| self.is_low_point(*p))
            .collect_vec()
    }

    fn risk_level(&self, position: Position) -> u32 {
        self.get_height(position).unwrap() + 1
    }

    fn basin_size(&self, initial_position: Position) -> usize {
        let mut visited = HashSet::new();
        let mut frontier = vec![initial_position];
        while let Some(current_position) = frontier.pop() {
            if matches!(self.get_height(current_position), None | Some(9)) {
                continue;
            }

            if !visited.insert(current_position) {
                continue;
            }

            frontier.extend(
                Direction::all()
                    .into_iter()
                    .flat_map(|d| current_position.step(d)),
            )
        }

        visited.len()
    }

    fn sum_of_risk_levels(&self) -> u32 {
        self.low_points()
            .into_iter()
            .map(|p| self.risk_level(p))
            .sum()
    }

    fn product_of_basin_sizes(&self) -> usize {
        self.low_points()
            .into_iter()
            .map(|p| self.basin_size(p))
            .k_largest(3)
            .product()
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let grid = Grid::new(&input);
    let output_1 = grid.sum_of_risk_levels();
    let output_2 = grid.product_of_basin_sizes();

    println!("part 1: {output_1} part 2: {output_2}")
}
