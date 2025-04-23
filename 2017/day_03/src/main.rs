use std::{collections::HashMap, fs::read_to_string, iter};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(self, direction: Direction) -> Self {
        let (dx, dy) = match direction {
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
        };

        let (x, y) = (self.x + dx, self.y + dy);
        Position { x, y }
    }

    fn get_surrounding(self) -> Vec<Self> {
        (-1..=1)
            .flat_map(|x| (-1..=1).map(move |y| (x, y)))
            .filter(|&(x, y)| x != 0 || y != 0)
            .map(|(dx, dy)| (self.x + dx, self.y + dy))
            .map(|(x, y)| Self { x, y })
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn positions() -> impl Iterator<Item = Position> {
    (1..)
        .flat_map(|n| [n, n])
        .zip(iter::successors(Some(Direction::Right), |d| {
            Some(d.rotate_right())
        }))
        .flat_map(|(n, d)| vec![d; n])
        .scan(Position::default(), |p, d| {
            let output = Some(*p);
            *p = p.step(d);
            output
        })
}

fn values_written() -> impl Iterator<Item = u32> {
    positions().skip(1).scan(
        HashMap::from_iter([(Position::default(), 1)]),
        |map: &mut HashMap<Position, u32>, p| {
            let value = p.get_surrounding().iter().filter_map(|p| map.get(p)).sum();
            map.insert(p, value);
            Some(value)
        },
    )
}

fn main() {
    let input: usize = read_to_string("input").unwrap().trim().parse().unwrap();
    let position = positions().nth(input - 1).unwrap();
    let output_1 = position.x.abs() + position.y.abs();
    let output_2 = values_written().find(|&v| v > input as u32).unwrap();

    println!("part 1: {output_1} part 2: {output_2}")
}
