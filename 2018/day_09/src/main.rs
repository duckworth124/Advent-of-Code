use std::{collections::VecDeque, fs::read_to_string};

struct State {
    marbles: VecDeque<u32>,
    scores: Vec<u32>,
    next: u32,
}

impl State {
    fn step(&mut self) {
        if self.next % 23 == 0 {
            let player = self.next as usize % self.scores.len();
            self.scores[player] += self.next;
            self.marbles.rotate_right(7);
            let marble = self.marbles.pop_back().unwrap();
            self.scores[player] += marble;
            self.marbles.rotate_left(1);
            self.next += 1;
            return;
        }
        self.marbles.rotate_left(1);
        self.marbles.push_back(self.next);
        self.next += 1;
    }
}

fn solve(input: &str) -> (u32, u32) {
    let players: usize = input.split_once(' ').unwrap().0.parse().unwrap();
    let max_marble: u32 = input
        .split_once("worth ")
        .unwrap()
        .1
        .split_once(' ')
        .unwrap()
        .0
        .parse()
        .unwrap();

    let mut state = State {
        marbles: VecDeque::from([0]),
        scores: vec![0; players],
        next: 1,
    };

    for _ in 0..max_marble {
        state.step();
    }

    let output_1 = state.scores.iter().copied().max().unwrap();

    for _ in 0..max_marble * 99 {
        state.step();
    }

    let output_2 = state.scores.iter().copied().max().unwrap();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
