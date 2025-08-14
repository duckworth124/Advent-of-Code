use std::fs::read_to_string;

struct State {
    numbers: Vec<usize>,
    first: usize,
    second: usize,
}

impl State {
    fn step(&mut self) {
        let first = self.numbers[self.first];
        let second = self.numbers[self.second];
        let new = first + second;
        if new >= 10 {
            self.numbers.extend([new / 10, new % 10]);
        } else {
            self.numbers.push(new);
        }
        self.first += first + 1;
        self.first %= self.numbers.len();
        self.second += second + 1;
        self.second %= self.numbers.len();
    }

    fn simulate(&mut self, target_len: usize) {
        while self.numbers.len() < target_len {
            self.step();
        }
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut state = State {
        numbers: vec![3, 7],
        first: 0,
        second: 1,
    };
    let target: usize = input.trim().parse().unwrap();
    state.simulate(target + 10);
    let output_1 = state
        .numbers
        .iter()
        .rev()
        .take(10)
        .rev()
        .copied()
        .fold(0, |acc, x| acc * 10 + x);
    let digits: Vec<usize> = target
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut i = 0;
    let output_2 = loop {
        if state.numbers[i..].starts_with(&digits) {
            break i;
        }

        if state.numbers[i..].len() >= digits.len() {
            i += 1;
            continue;
        }

        state.step();
    };

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
