use std::{collections::HashSet, fs::read_to_string};
use winnow::{Parser, token::any};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    Idle,
    Working(char, u8),
}

struct State<const N: usize> {
    rules: Vec<(char, char)>,
    completed: Vec<char>,
    not_started: HashSet<char>,
    workers: [Status; N],
}

impl<const N: usize> State<N> {
    fn new(rules: Vec<(char, char)>) -> Self {
        let not_started = rules.iter().copied().flat_map(<[char; 2]>::from).collect();

        Self {
            rules,
            not_started,
            completed: vec![],
            workers: [Status::Idle; N],
        }
    }

    fn update(&mut self) -> bool {
        for worker in &mut self.workers {
            if let Status::Working(c, time) = *worker {
                if time == 0 {
                    *worker = Status::Idle;
                    self.completed.push(c);
                    self.rules.retain(|(x, _)| *x != c);
                }
            }
        }

        for worker in &mut self.workers {
            if *worker == Status::Idle {
                let next = match ('A'..='Z')
                    .filter(|c| self.not_started.contains(c))
                    .find(|c| self.rules.iter().all(|(_, x)| x != c))
                {
                    Some(c) => c,
                    None => break,
                };
                self.not_started.remove(&next);
                let extra_time = if N == 5 { 60 } else { 0 };
                let time = extra_time + 1 + (next as u8 - b'A');
                *worker = Status::Working(next, time);
            }
        }

        for worker in &mut self.workers {
            if let Status::Working(_, ref mut time) = *worker {
                *time -= 1;
            }
        }

        self.not_started.is_empty() && self.workers.iter().all(|s| *s == Status::Idle)
    }
}

fn topological_sort(mut rules: Vec<(char, char)>) -> Vec<char> {
    let mut output = vec![];
    let mut elements: HashSet<char> = rules.iter().copied().flat_map(<[char; 2]>::from).collect();

    loop {
        let next = match ('A'..='Z')
            .filter(|c| elements.contains(c))
            .find(|c| rules.iter().all(|(_, x)| x != c))
        {
            Some(x) => x,
            None => return output,
        };

        output.push(next);
        rules.retain(|(c, _)| *c != next);
        elements.remove(&next);
    }
}

fn process_line(line: &str) -> (char, char) {
    (
        "Step ",
        any::<&str, ()>,
        " must be finished before step ",
        any,
        " can begin.",
    )
        .map(|(_, c1, _, c2, _)| (c1, c2))
        .parse(line)
        .unwrap()
}

fn solve(input: &str) -> (String, u32) {
    let rules: Vec<(char, char)> = input.lines().map(process_line).collect();
    let output_1 = topological_sort(rules.clone()).into_iter().collect();
    let mut output_2 = 0;
    let mut state = State::<5>::new(rules);
    while !state.update() {
        output_2 += 1;
    }
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
