use std::{fmt::Display, fs::read_to_string};

#[derive(Clone)]
struct State {
    cups: Vec<u32>,
}

impl State {
    fn step(&mut self) {
        let to_move = &self.cups[..3];
        let current = self.cups[8];
        let mut destination = current - 1;
        loop {
            if destination == 0 {
                destination = 9;
                continue;
            }
            if to_move.contains(&destination) {
                destination -= 1;
                continue;
            }
            break;
        }
        let i = self.cups.iter().position(|x| *x == destination).unwrap();
        self.cups = [&self.cups[3..=i], to_move, &self.cups[i + 1..]].concat();
        self.cups.rotate_left(1);
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self
            .cups
            .iter()
            .copied()
            .cycle()
            .skip_while(|c| *c != 1)
            .skip(1)
            .take(8)
        {
            write!(f, "{c}")?
        }
        Ok(())
    }
}

fn solve(input: &str) -> String {
    let cups: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut state = State { cups };
    state.cups.rotate_left(1);
    for _ in 0..100 {
        state.step();
    }

    let output_1 = state.to_string();

    output_1
}

fn main() {
    let input = read_to_string("input").unwrap();
    let output_1 = solve(input.trim());
    println!("part 1: {output_1}")
}
