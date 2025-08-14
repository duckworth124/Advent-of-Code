use itertools::Itertools;
use std::fs::read_to_string;
use std::ops::Add;

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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let directions = input
        .chars()
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("unexpected char {c:?}"),
        })
        .collect_vec();

    let output_1 = directions
        .iter()
        .scan(Position::default(), |p, &d| {
            *p = p.step(d);
            Some(*p)
        })
        .chain([Position::default()])
        .unique()
        .count();

    let output_2 = directions
        .iter()
        .chunks(2)
        .into_iter()
        .scan(
            (Position::default(), Position::default()),
            |(p1, p2), mut ds| {
                *p1 = p1.step(*ds.next().unwrap());
                *p2 = p2.step(*ds.next().unwrap());
                Some([*p1, *p2])
            },
        )
        .flatten()
        .unique()
        .count();

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
