use std::fs::read_to_string;

struct SimpleState {
    buffer_len: usize,
    value_after_0: u32,
    current_value: u32,
    index: usize,
}

impl SimpleState {
    fn step(&mut self, steps: usize) {
        self.index += steps;
        self.index %= self.buffer_len
    }

    fn insert(&mut self) {
        if self.index == 0 {
            self.value_after_0 = self.current_value;
        }
        self.buffer_len += 1;
        self.current_value += 1;
        self.index += 1
    }

    fn update(&mut self, steps: usize) {
        self.step(steps);
        self.insert();
    }

    fn run(&mut self, steps: usize, num_cycles: u32) {
        for _ in 0..num_cycles {
            self.update(steps);
        }
    }
}

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

    fn run(&mut self, steps: usize, num_cycles: u32) {
        for _ in 0..num_cycles {
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
    state.run(steps, 2017);
    let output_1 = state.buffer[(state.index + 1) % state.buffer.len()];

    let mut state = SimpleState {
        buffer_len: 2,
        current_value: 2,
        index: 1,
        value_after_0: 1,
    };
    state.run(steps, 50_000_000 - 1);
    let output_2 = state.value_after_0;

    println!("part 1: {output_1} part 2: {output_2}")
}
