use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs::read_to_string,
};

struct State {
    plants: BTreeSet<i64>,
    rules: HashMap<[bool; 5], bool>,
    offset: i64,
}

impl State {
    fn step(&mut self) {
        let min = *self.plants.iter().min().unwrap();
        let max = *self.plants.iter().max().unwrap();
        let plants = (min - 2..=max + 2)
            .map(|i| {
                (
                    i,
                    [i - 2, i - 1, i, i + 1, i + 2].map(|i| self.plants.contains(&i)),
                )
            })
            .filter(|(_, a)| self.rules[a])
            .map(|(i, _)| i)
            .collect();

        self.plants = plants
    }

    fn centre(&mut self) {
        let min = *self.plants.iter().min().unwrap();
        self.plants = self.plants.iter().copied().map(|x| x - min).collect();
        self.offset += min;
    }

    fn simulate(&mut self, mut steps: usize) {
        let mut seen = BTreeMap::new();
        while steps > 0 {
            self.step();
            self.centre();
            steps -= 1;
            if let Some((old_steps, old_offset)) =
                seen.insert(self.plants.clone(), (steps, self.offset))
            {
                let offset_change = self.offset - old_offset;
                let steps_change = old_steps - steps;
                let num_cycles = steps / steps_change;
                self.offset += offset_change * num_cycles as i64;
                steps %= steps_change;
            }
        }
    }

    fn sum(&self) -> i64 {
        self.plants.iter().map(|x| x + self.offset).sum()
    }
}

fn solve(input: &str) -> (i64, i64) {
    let plants = input
        .lines()
        .next()
        .unwrap()
        .strip_prefix("initial state: ")
        .unwrap()
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == '#')
        .map(|(x, _)| x as i64)
        .collect();

    let rules = input
        .lines()
        .skip(2)
        .map(|s| s.split_once(" => ").unwrap())
        .map(|(l, r)| (l.chars().map(|c| c == '#').collect(), r == "#"))
        .map(|(v, b): (Vec<bool>, bool)| (v.try_into().unwrap(), b))
        .collect();

    let mut state = State {
        plants,
        rules,
        offset: 0,
    };

    state.simulate(20);
    let output_1 = state.sum();
    state.simulate(50_000_000_000 - 20);

    let output_2 = state.sum();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
