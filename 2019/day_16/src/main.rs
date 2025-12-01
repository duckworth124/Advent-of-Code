use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs::read_to_string;
use tqdm::Iter;

struct State(Vec<i64>);

impl State {
    fn step(&mut self) {
        let psa: Vec<i64> = std::iter::once(0)
            .chain(self.0.iter().copied().scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            }))
            .collect();

        let new = (1..=self.0.len())
            .into_par_iter()
            .map(|n| ranges(n, self.0.len()))
            .map(|v| {
                v.into_iter()
                    .map(|(l, r)| psa[r] - psa[l])
                    .rev()
                    .fold(0, |acc, x| x - acc)
            })
            .map(|x| x.abs() % 10)
            .collect();
        self.0 = new
    }
}

fn ranges(n: usize, len: usize) -> Vec<(usize, usize)> {
    (1..)
        .step_by(2)
        .map(|x| (x * n - 1, x * n + n - 1))
        .take_while(|(low, _)| *low < len)
        .map(|(low, high)| (low, high.min(len)))
        .collect()
}

fn solve(input: &str) -> (String, String) {
    let mut state = State(
        input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect(),
    );
    for _ in 0..100 {
        state.step();
    }
    let output_1 = state.0[..8].iter().map(|n| n.to_string()).collect();
    dbg!(&output_1);
    let input = [input; 10_000].concat();
    let mut state = State(
        input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect(),
    );
    let offset: usize = input[..7].parse().unwrap();
    for _ in (0..100).tqdm() {
        state.step();
    }
    let s: String = state.0.iter().copied().map(|d| d.to_string()).collect();
    let output_2 = s[offset..][..8].to_string();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(input.trim());
    println!("part 1: {output_1} part 2: {output_2}")
}
