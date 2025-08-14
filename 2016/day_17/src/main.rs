use md5::compute;
use std::{collections::VecDeque, fs::read_to_string};

#[derive(Clone)]
struct State<'a> {
    position: (u8, u8),
    path: String,
    salt: &'a str,
}

impl State<'_> {
    fn possible_next(self) -> Vec<Self> {
        let hash = format!("{:x}", compute(format!("{}{}", self.salt, self.path)));
        hash.chars()
            .take(4)
            .zip([
                Some(self.position.0).zip(self.position.1.checked_sub(1)),
                Some(self.position.0).zip(Some(self.position.1 + 1).filter(|&x| x < 4)),
                (self.position.0.checked_sub(1)).zip(Some(self.position.1)),
                Some(self.position.0 + 1)
                    .filter(|&x| x < 4)
                    .zip(Some(self.position.1)),
            ])
            .zip("UDLR".chars())
            .filter_map(|((c, o), d)| Some(c).zip(o).zip(Some(d)))
            .filter(|((c, _), _)| "bcdef".contains(*c))
            .map(|((_, p), c)| Self {
                position: p,
                salt: self.salt,
                path: self.path.clone() + &c.to_string(),
            })
            .collect()
    }

    fn shortest_path(self) -> String {
        let mut frontier = VecDeque::from([self]);

        loop {
            let current = frontier.pop_front().unwrap();
            if current.position == (3, 3) {
                return current.path;
            }
            frontier.extend(current.possible_next());
        }
    }

    fn longest_path(self) -> usize {
        let mut output = 0;
        let mut frontier = VecDeque::from([self]);

        loop {
            let current = if let Some(x) = frontier.pop_front() {
                x
            } else {
                break output;
            };
            if current.position == (3, 3) {
                output = output.max(current.path.len());
                continue;
            }
            frontier.extend(current.possible_next());
        }
    }
}

fn solve(input: &str) -> (String, usize) {
    let salt = input.trim();
    let state = State {
        position: (0, 0),
        path: String::new(),
        salt,
    };
    let output_1 = state.clone().shortest_path();
    let output_2 = state.longest_path();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
