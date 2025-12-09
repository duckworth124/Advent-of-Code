use std::{fs::read_to_string, time::Instant};

#[derive(Clone, Copy)]
struct Function {
    times: i128,
    add: i128,
}

impl Function {
    const fn then(self, other: Self, modulo: i128) -> Self {
        let times = (self.times * other.times).rem_euclid(modulo);
        let add = (other.times * self.add + other.add).rem_euclid(modulo);
        Self { times, add }
    }

    const fn inverse(self, modulo: i128) -> Self {
        let mut i = 1;
        while i % self.times != 0 {
            i += modulo;
        }
        let times = i / self.times;
        let add = -times * self.add;
        Self { times, add }
    }

    const fn id() -> Self {
        Self { times: 1, add: 0 }
    }

    const fn pow(mut self, mut n: u64, modulo: i128) -> Self {
        let mut x = Self::id();
        while n > 0 {
            if n % 2 == 0 {
                n /= 2;
                self = self.then(self, modulo)
            } else {
                x = x.then(self, modulo);
                n -= 1
            }
        }
        x
    }

    const fn apply(self, x: i128, modulo: i128) -> i128 {
        (x * self.times + self.add).rem_euclid(modulo)
    }
}

const NUM_CYCLES: u64 = 101741582076661;
const DECK_SIZE: i128 = 119315717514047;
const SMALL_DECK_SIZE: i128 = 10007;

fn solve(input: &str) -> (i128, i128) {
    let functions: Vec<Function> = input
        .lines()
        .map(|l| {
            if l == "deal into new stack" {
                Function { times: -1, add: -1 }
            } else if let Some(s) = l.strip_prefix("cut ") {
                Function {
                    add: -s.parse::<i128>().unwrap(),
                    times: 1,
                }
            } else if let Some(s) = l.strip_prefix("deal with increment ") {
                Function {
                    times: s.parse().unwrap(),
                    add: 0,
                }
            } else {
                panic!("unrecognized line: {l:?}")
            }
        })
        .collect();
    let function_1 = functions
        .iter()
        .copied()
        .fold(Function::id(), |f, g| f.then(g, SMALL_DECK_SIZE));

    let function_2 = functions
        .iter()
        .copied()
        .rev()
        .map(|f| f.inverse(DECK_SIZE))
        .fold(Function::id(), |f, g| f.then(g, DECK_SIZE))
        .pow(NUM_CYCLES, DECK_SIZE);

    let output_1 = function_1.apply(2019, SMALL_DECK_SIZE);
    let output_2 = function_2.apply(2020, DECK_SIZE);
    (output_1, output_2)
}

fn main() {
    let time = Instant::now();
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(input.trim());
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
