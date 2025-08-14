use std::fs::read_to_string;

use itertools::{Itertools, iproduct};

#[derive(PartialEq, Eq)]
enum Direction {
    Right,
    Down,
}

struct State(Vec<Vec<Option<Direction>>>);

impl State {
    fn step(&mut self) -> bool {
        let width = self.0[0].len();
        let height = self.0.len();
        let mut output = false;

        let to_move = iproduct!(0..width, 0..height)
            .filter(|&(x, y)| self.0[y][x] == Some(Direction::Right))
            .filter(|&(x, y)| {
                let new_x = if x + 1 == width { 0 } else { x + 1 };
                self.0[y][new_x].is_none()
            })
            .collect_vec();
        output |= !to_move.is_empty();

        for (x, y) in to_move {
            self.0[y][x] = None;
            let new_x = if x + 1 == width { 0 } else { x + 1 };
            self.0[y][new_x] = Some(Direction::Right)
        }

        let to_move = iproduct!(0..width, 0..height)
            .filter(|&(x, y)| self.0[y][x] == Some(Direction::Down))
            .filter(|&(x, y)| {
                let new_y = if y + 1 == height { 0 } else { y + 1 };
                self.0[new_y][x].is_none()
            })
            .collect_vec();

        output |= !to_move.is_empty();

        for (x, y) in to_move {
            self.0[y][x] = None;
            let new_y = if y + 1 == height { 0 } else { y + 1 };
            self.0[new_y][x] = Some(Direction::Down)
        }

        output
    }

    fn run(&mut self) -> usize {
        let mut output = 1;
        while self.step() {
            output += 1;
        }
        output
    }
}

fn solve(input: &str) -> usize {
    let mut state = State(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => None,
                        '>' => Some(Direction::Right),
                        'v' => Some(Direction::Down),
                        _ => panic!("unrecognized char: {c:?}"),
                    })
                    .collect()
            })
            .collect(),
    );

    state.run()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let output = solve(&input);
    println!("part 1: {output}")
}
