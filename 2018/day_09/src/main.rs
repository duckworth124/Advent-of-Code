use std::fs::read_to_string;
use tqdm::Iter;

struct State {
    marbles: Vec<u32>,
    scores: Vec<u32>,
    current_marble_position: usize,
    next_marble_value: u32,
    player: usize,
}

impl State {
    fn step(&mut self) {
        if self.next_marble_value <= 1 {
            self.marbles.push(self.next_marble_value);
            self.current_marble_position = self.marbles.len() - 1;
        } else if self.next_marble_value % 23 == 0 {
            self.scores[self.player] += self.next_marble_value;

            let next_index =
                (self.current_marble_position + self.marbles.len() - 7) % self.marbles.len();

            self.scores[self.player] += self.marbles[next_index];
            self.marbles.remove(next_index);
            self.current_marble_position = next_index;
        } else {
            let next_index = (self.current_marble_position + 2) % self.marbles.len();
            self.marbles.insert(next_index, self.next_marble_value);
            self.current_marble_position = next_index;
        }

        self.next_marble_value += 1;
        self.player += 1;
        self.player %= self.scores.len();
    }

    fn new(num_players: usize) -> Self {
        Self {
            marbles: vec![],
            current_marble_position: 0,
            player: 0,
            next_marble_value: 0,
            scores: vec![0; num_players],
        }
    }
}

fn solve(input: &str) -> (u32, u32) {
    let num_players: usize = input.split_once(' ').unwrap().0.parse().unwrap();
    let max_marble_value: u32 = input
        .split_once("worth ")
        .unwrap()
        .1
        .split_once(' ')
        .unwrap()
        .0
        .parse()
        .unwrap();

    let mut state = State::new(num_players);
    for _ in 0..=max_marble_value {
        state.step();
    }

    let output_1 = *state.scores.iter().max().unwrap();
    dbg!(state.scores);

    let mut state = State::new(num_players);
    for _ in (0..=max_marble_value * 100).tqdm() {
        state.step();
    }
    let output_2 = *state.scores.iter().max().unwrap();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
