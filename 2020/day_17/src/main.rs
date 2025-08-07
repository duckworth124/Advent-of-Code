use itertools::{Itertools, iproduct};
use std::{collections::HashSet, fs::read_to_string};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32, i32);

impl Point {
    fn neighbours(self) -> Vec<Self> {
        let Self(x, y, z) = self;
        iproduct!(-1..=1, -1..=1, -1..=1)
            .filter(|&(x, y, z)| x != 0 || y != 0 || z != 0)
            .map(|(i, j, k)| Self(x + i, y + j, z + k))
            .collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point4(i32, i32, i32, i32);
impl Point4 {
    fn neighbours(self) -> Vec<Self> {
        let Self(x, y, z, w) = self;
        iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
            .filter(|&(x, y, z, w)| x != 0 || y != 0 || z != 0 || w != 0)
            .map(|(i, j, k, l)| Self(x + i, y + j, z + k, w + l))
            .collect()
    }
}

struct Grid(HashSet<Point>);

impl Grid {
    fn step(&mut self) {
        let new = self
            .0
            .iter()
            .flat_map(|p| p.neighbours())
            .unique()
            .filter(|p| self.is_alive_next_step(*p))
            .collect();
        self.0 = new
    }

    fn is_alive_next_step(&self, point: Point) -> bool {
        let alive_neighbour_count = point
            .neighbours()
            .into_iter()
            .filter(|p| self.0.contains(p))
            .count();

        match alive_neighbour_count {
            2 => self.0.contains(&point),
            3 => true,
            _ => false,
        }
    }
}

struct Grid4(HashSet<Point4>);

impl Grid4 {
    fn step(&mut self) {
        let new = self
            .0
            .iter()
            .flat_map(|p| p.neighbours())
            .unique()
            .filter(|p| self.is_alive_next_step(*p))
            .collect();
        self.0 = new
    }

    fn is_alive_next_step(&self, point: Point4) -> bool {
        let alive_neighbour_count = point
            .neighbours()
            .into_iter()
            .filter(|p| self.0.contains(p))
            .count();

        match alive_neighbour_count {
            2 => self.0.contains(&point),
            3 => true,
            _ => false,
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut grid = Grid(
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| l.char_indices().map(move |(x, c)| (x, y, c)))
            .filter(|(_, _, c)| *c == '#')
            .map(|(x, y, _)| Point(x as i32, y as i32, 0))
            .collect(),
    );

    for _ in 0..6 {
        grid.step();
    }

    let output_1 = grid.0.len();

    let mut grid = Grid4(
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| l.char_indices().map(move |(x, c)| (x, y, c)))
            .filter(|(_, _, c)| *c == '#')
            .map(|(x, y, _)| Point4(x as i32, y as i32, 0, 0))
            .collect(),
    );

    for _ in 0..6 {
        grid.step();
    }

    let output_2 = grid.0.len();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
