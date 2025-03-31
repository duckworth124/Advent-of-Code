use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    iter::repeat_n,
    usize,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(self, direction: Direction) -> Self {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_instruction(input: &str) -> (Direction, usize) {
    let d = match input.chars().next().unwrap() {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("unrecognized character: {}", input.chars().next().unwrap()),
    };
    let n = input[1..].parse().unwrap();
    (d, n)
}

fn get_visited(instructions: &[(Direction, usize)]) -> HashMap<Position, usize> {
    instructions
        .iter()
        .flat_map(|&(d, n)| repeat_n(d, n))
        .scan((Position::default(), 0), |(pos, n), d| {
            *pos = pos.step(d);
            *n += 1;
            Some((*pos, *n))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

fn solve(input: &str) -> (u32, usize) {
    let wire_1: Vec<(Direction, usize)> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(get_instruction)
        .collect();
    let wire_2: Vec<(Direction, usize)> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(get_instruction)
        .collect();

    let path_1 = get_visited(&wire_1);
    let path_2 = get_visited(&wire_2);

    let output_1 = path_1
        .keys()
        .filter(|k| path_2.contains_key(k))
        .map(|p| p.x.abs() + p.y.abs())
        .min()
        .unwrap() as u32;

    let output_2 = path_1
        .iter()
        .filter_map(|(k, v)| Some((path_2.get(k)?, v)))
        .map(|(&x, &y)| x + y)
        .min()
        .unwrap();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();

    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
