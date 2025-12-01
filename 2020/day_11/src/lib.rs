use std::fmt::Display;
use std::fs::read_to_string;
use std::iter::successors;
use std::ops::{Add, Neg, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(self, direction: Direction) -> Self {
        self + direction.into()
    }

    fn surrounding(self) -> [Self; 8] {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .map(|(x, y)| Self { x, y })
        .map(|p| self + p)
    }
}

impl From<Direction> for Position {
    fn from(value: Direction) -> Self {
        let (x, y) = match value {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (1, -1),
            Direction::DownLeft => (-1, 1),
            Direction::DownRight => (1, 1),
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
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    const fn all() -> [Self; 8] {
        [
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
            Self::UpRight,
            Self::UpLeft,
            Self::DownRight,
            Self::DownLeft,
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Occupied,
    Vacant,
    Floor,
}

#[derive(Clone)]
struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn new(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            'L' => Tile::Vacant,
                            '#' => Tile::Occupied,
                            '.' => Tile::Floor,
                            _ => panic!("unrecognized character: {c}"),
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn get(&self, position: Position) -> Option<Tile> {
        let x: usize = position.x.try_into().ok()?;
        let y: usize = position.y.try_into().ok()?;
        self.0.get(y)?.get(x).copied()
    }

    fn count_occupied_surrounding(&self, position: Position) -> usize {
        position
            .surrounding()
            .into_iter()
            .flat_map(|p| self.get(p))
            .filter(|t| *t == Tile::Occupied)
            .count()
    }

    fn step(&mut self) -> bool {
        let mut grid = self.0.clone();
        let mut changed = false;
        grid.iter_mut()
            .enumerate()
            .flat_map(|(y, r)| r.iter_mut().enumerate().map(move |(x, t)| (x, y, t)))
            .map(|(x, y, t)| (x as i32, y as i32, t))
            .map(|(x, y, t)| (Position { x, y }, t))
            .for_each(|(p, t)| {
                let surrounding = self.count_occupied_surrounding(p);

                match *t {
                    Tile::Floor => {}
                    Tile::Vacant => {
                        if surrounding == 0 {
                            *t = Tile::Occupied;
                            changed = true;
                        }
                    }
                    Tile::Occupied => {
                        if surrounding >= 4 {
                            *t = Tile::Vacant;
                            changed = true;
                        }
                    }
                }
            });
        self.0 = grid;
        changed
    }

    fn scan(&self, position: Position, direction: Direction) -> Vec<Tile> {
        successors(Some(position.step(direction)), |p| Some(p.step(direction)))
            .map(|p| self.get(p))
            .take_while(|o| o.is_some())
            .flatten()
            .collect()
    }

    fn get_far(&self, position: Position, direction: Direction) -> Option<Tile> {
        self.scan(position, direction)
            .into_iter()
            .find(|t| *t != Tile::Floor)
    }

    fn count_occupied_surrounding_far(&self, position: Position) -> usize {
        Direction::all()
            .into_iter()
            .filter_map(|d| self.get_far(position, d))
            .filter(|t| *t == Tile::Occupied)
            .count()
    }

    fn step_part_2(&mut self) -> bool {
        let mut grid = self.0.clone();
        let mut changed = false;
        grid.iter_mut()
            .enumerate()
            .flat_map(|(y, r)| r.iter_mut().enumerate().map(move |(x, t)| (x, y, t)))
            .map(|(x, y, t)| (x as i32, y as i32, t))
            .map(|(x, y, t)| (Position { x, y }, t))
            .for_each(|(p, t)| {
                let surrounding = self.count_occupied_surrounding_far(p);

                match *t {
                    Tile::Floor => {}
                    Tile::Vacant => {
                        if surrounding == 0 {
                            *t = Tile::Occupied;
                            changed = true;
                        }
                    }
                    Tile::Occupied => {
                        if surrounding >= 5 {
                            *t = Tile::Vacant;
                            changed = true;
                        }
                    }
                }
            });

        self.0 = grid;
        changed
    }

    fn count_occupied(&self) -> usize {
        self.0
            .iter()
            .flatten()
            .filter(|t| **t == Tile::Occupied)
            .count()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in &self.0 {
            for t in r {
                let output = match t {
                    Tile::Occupied => '#',
                    Tile::Vacant => 'L',
                    Tile::Floor => '.',
                };
                write!(f, "{output}")?
            }
            writeln!(f)?
        }

        Ok(())
    }
}

pub fn solve(path: &str) -> (usize, usize) {
    println!("{path}");
    let input = read_to_string(path).unwrap();
    let mut grid = Grid::new(&input);
    let mut grid_2 = grid.clone();
    while grid.step() {}
    grid.count_occupied();
    while grid_2.step_part_2() {}
    let output_1 = grid.count_occupied();
    let output_2 = grid_2.count_occupied();
    (output_1, output_2)
}
