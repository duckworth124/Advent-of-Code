use std::{collections::HashSet, fs::read_to_string};

struct TuringMachine {
    tape: HashSet<i32>,
    position: i32,
    current_state: usize,
    states: Vec<State>,
}

impl TuringMachine {
    fn step(&mut self) {
        let state = self.states[self.current_state];
        let instructions = if self.tape.contains(&self.position) {
            state.if_1
        } else {
            state.if_0
        };
        self.apply_instructions(instructions);
    }

    fn apply_instructions(&mut self, instructions: Instructions) {
        if instructions.to_write {
            self.tape.insert(self.position);
        } else {
            self.tape.remove(&self.position);
        }

        match instructions.movement {
            Movement::Left => self.position -= 1,
            Movement::Right => self.position += 1,
        };

        self.current_state = instructions.next_state
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    if_0: Instructions,
    if_1: Instructions,
}

impl State {
    fn parse(input: &str) -> Self {
        let if_0 = Instructions::parse(input.split_once('-').unwrap().1.split_once('I').unwrap().0);
        let if_1 = Instructions::parse(input.split(":\n").last().unwrap());
        Self { if_0, if_1 }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instructions {
    to_write: bool,
    movement: Movement,
    next_state: usize,
}

impl Instructions {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let to_write = lines.next().unwrap().ends_with("1.");
        let movement = if lines.next().unwrap().ends_with("right.") {
            Movement::Right
        } else {
            Movement::Left
        };
        let next_state = lines
            .next()
            .unwrap()
            .strip_suffix('.')
            .unwrap()
            .chars()
            .last()
            .unwrap() as usize
            - 'A' as usize;

        Self {
            to_write,
            movement,
            next_state,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Movement {
    Left,
    Right,
}

fn solve(input: &str) -> usize {
    let states: Vec<State> = input.split("\n\n").skip(1).map(State::parse).collect();
    let mut tm = TuringMachine {
        states,
        current_state: 0,
        tape: HashSet::new(),
        position: 0,
    };

    let num_steps: u32 = input
        .lines()
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();

    for _ in 0..num_steps {
        tm.step();
    }

    tm.tape.len()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let output = solve(&input);
    println!("part 1: {output}")
}
