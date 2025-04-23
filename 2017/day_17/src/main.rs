use std::fs::read_to_string;

struct State {
    buffer: Vec<u32>,
    index: usize,
    value: u32,
}

impl State {
    fn step(&mut self, steps: usize) {
        self.index += steps;
        self.index %= self.buffer.len();
    }

    fn insert(&mut self) {
        self.buffer.insert(self.index + 1, self.value);
        self.value += 1;
        self.index += 1;
    }

    fn update(&mut self, steps: usize) {
        self.step(steps);
        self.insert();
    }

    fn run(&mut self, steps: usize) {
        for _ in 0..2017 {
            self.update(steps);
        }
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let steps = input.trim().parse().unwrap();
    let mut state = State {
        buffer: vec![0],
        index: 0,
        value: 1,
    };
    state.run(steps);
    let output_1 = state.buffer[(state.index + 1) % state.buffer.len()];
    println!("part 1: {output_1}")
}
