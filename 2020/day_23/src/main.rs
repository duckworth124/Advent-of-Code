use itertools::Itertools;
use std::{fmt::Display, fs::read_to_string};

struct State {
    connections: Vec<usize>,
    current_cup: usize,
}

impl State {
    fn step(&mut self) {
        let first_removed = self.next(self.current_cup);
        let last_removed = self.next(self.next(first_removed));
        let after_removed = self.next(last_removed);
        let removed = [first_removed, self.next(first_removed), last_removed];
        let mut destination = self.current_cup - 1;
        loop {
            if destination == 0 {
                destination = self.connections.len() - 1;
                continue;
            }
            if removed.contains(&destination) {
                destination -= 1;
                continue;
            }
            break;
        }
        let after_destination = self.next(destination);
        self.connections[self.current_cup] = after_removed;
        self.connections[destination] = first_removed;
        self.connections[last_removed] = after_destination;
        self.current_cup = self.next(self.current_cup);
    }

    fn next(&self, from: usize) -> usize {
        self.connections[from]
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.next(1);
        while current != 1 {
            write!(f, "{current}")?;
            current = self.next(current)
        }
        Ok(())
    }
}

fn solve(input: &str) -> (String, usize) {
    let mut connections = vec![0; 10];
    for (l, r) in input
        .chars()
        .collect_vec()
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .circular_tuple_windows()
    {
        connections[l] = r;
    }
    let first = input.chars().next().unwrap() as usize - '0' as usize;
    let last = input.chars().last().unwrap() as usize - '0' as usize;
    let mut state = State {
        connections: connections.clone(),
        current_cup: first,
    };
    for _ in 0..100 {
        state.step();
    }
    let output_1 = state.to_string();

    connections[last] = 10;
    connections.extend(11..=1_000_000);
    connections.push(first);
    state = State {
        current_cup: first,
        connections,
    };

    for _ in 0..10_000_000 {
        state.step();
    }

    let l = state.next(1);
    let r = state.next(l);
    let output_2 = l * r;

    (output_1, output_2)
}
fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(input.trim());
    println!("part 1: {output_1} part 2: {output_2}")
}
