use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct State {
    index: u32,
    seen: HashMap<u32, u32>,
    last: u32,
}

impl State {
    fn next(&mut self) -> u32 {
        let next = self
            .seen
            .get(&self.last)
            .copied()
            .map(|x| self.index - x)
            .unwrap_or_default();
        self.seen.insert(self.last, self.index);
        self.index += 1;
        self.last = next;
        next
    }

    fn new(numbers: &[u32]) -> Self {
        let index = numbers.len() as u32 - 1;
        let last = *numbers.last().unwrap();
        let seen = numbers
            .iter()
            .take(numbers.len() - 1)
            .enumerate()
            .map(|(x, y)| (*y, x as u32))
            .collect();

        Self { index, last, seen }
    }

    fn nth(&mut self, n: u32) -> u32 {
        while self.index < n - 1 {
            self.next();
        }
        self.last
    }
}

fn solve(input: &str) -> (u32, u32) {
    let numbers: Vec<u32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut state = State::new(&numbers);
    let output_1 = state.nth(2020);
    let mut state = State::new(&numbers);
    let output_2 = state.nth(30_000_000);
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
