use std::{collections::HashMap, fs::read_to_string};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State(Vec<char>);

impl State {
    fn spin(&mut self, n: usize) {
        self.0.rotate_right(n);
    }

    fn exchange(&mut self, n: usize, m: usize) {
        let c = self.0[m];
        self.0[m] = self.0[n];
        self.0[n] = c;
    }

    fn partner(&mut self, x: char, y: char) {
        let n = self.0.iter().position(|&c| c == x).unwrap();
        let m = self.0.iter().position(|&c| c == y).unwrap();
        self.exchange(n, m);
    }

    fn dance(&mut self, input: &str) {
        for s in input.trim().split(',') {
            let c = s.chars().next().unwrap();
            match c {
                's' => {
                    let n = s[1..].parse().unwrap();
                    self.spin(n)
                }
                'x' => {
                    let (l, r) = s[1..].split_once('/').unwrap();
                    let (n, m) = (l.parse().unwrap(), r.parse().unwrap());
                    self.exchange(n, m);
                }
                'p' => {
                    let (l, r) = s[1..].split_once('/').unwrap();
                    let (x, y) = (l.parse().unwrap(), r.parse().unwrap());
                    self.partner(x, y);
                }
                _ => panic!("unrecognized char: {c}"),
            }
        }
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let mut state = State(('a'..='p').collect());
    state.dance(&input);
    let output_1: String = state.0.iter().collect();
    let mut remaining = 999_999_999;
    let mut seen: HashMap<State, u32> = HashMap::new();
    while remaining > 0 {
        remaining -= 1;
        state.dance(&input);
        if let Some(&n) = seen.get(&state) {
            let cycle_len = n - remaining;
            remaining %= cycle_len;
            continue;
        }
        seen.insert(state.clone(), remaining);
    }
    let output_2: String = state.0.iter().collect();
    println!("part 1: {output_1} part 2: {output_2}")
}
