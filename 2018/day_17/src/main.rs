use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: u32,
    y: u32,
}

struct State {
    blocked: HashSet<Position>,
    water: HashSet<Position>,
    visited: HashSet<Position>,
}

impl State {
    fn spawn_water(&mut self, start: Position) -> bool {
        let mut current = start;
        loop {
            current.y += 1;

        }
    }
}

fn main() {
    println!("Hello, world!");
}
