use std::{collections::HashSet, fs::read_to_string};

#[derive(Clone, Copy)]
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

    fn rotate_left(self) -> Self {
        self.rotate_right().rotate_right().rotate_right()
    }

    fn to_coords(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

fn solve(instructions: &[(char, usize)]) -> (i32, i32) {
    let path: Vec<(i32, i32)> = instructions
        .iter()
        .scan(Direction::Up, |d, &(c, n)| {
            *d = match c {
                'R' => d.rotate_right(),
                'L' => d.rotate_left(),
                _ => panic!("unrecognized character : {c}"),
            };
            Some((*d, n))
        })
        .flat_map(|(d, n)| vec![d; n])
        .scan((0, 0), |(x, y), d| {
            let (dx, dy) = d.to_coords();
            *x += dx;
            *y += dy;
            Some((*x, *y))
        })
        .collect();

    let (x, y) = path.last().unwrap();
    let output_1 = x.abs() + y.abs();

    let mut visited = HashSet::new();

    let first_repeat = path.into_iter().find(|&p| !visited.insert(p)).unwrap();

    let (x, y) = first_repeat;
    let output_2 = x.abs() + y.abs();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let instructions = input
        .trim()
        .split(", ")
        .map(|s| {
            let (c, n) = s.split_at(1);
            (c.chars().next().unwrap(), n.parse().unwrap())
        })
        .collect::<Vec<_>>();

    let (output_1, output_2) = solve(&instructions);
    println!("part 1: {output_1} part 2: {output_2}")
}
