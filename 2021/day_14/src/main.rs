use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

struct InsertionMap(HashMap<[char; 2], char>);

impl InsertionMap {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .skip(2)
            .map(|line| {
                let left = line.chars().next().unwrap();
                let right = line.chars().nth(1).unwrap();
                let output = line.chars().last().unwrap();
                ([left, right], output)
            })
            .collect();

        Self(map)
    }
}

struct Polymer {
    char_count: HashMap<char, usize>,
    pair_count: HashMap<[char; 2], usize>,
    insertion_map: HashMap<[char; 2], char>,
}

impl Polymer {
    fn new(input: &str) -> Self {
        let elements: Vec<_> = input.lines().next().unwrap().chars().collect();
        let char_count = elements.iter().copied().counts();
        let pair_count = elements.windows(2).map(|v| [v[0], v[1]]).counts();
        let InsertionMap(insertion_map) = InsertionMap::new(input);

        Self {
            insertion_map,
            char_count,
            pair_count,
        }
    }

    fn get_difference(&self) -> usize {
        let counts = &self.char_count;
        match counts.values().minmax() {
            itertools::MinMaxResult::MinMax(l, h) => h - l,
            _ => 0,
        }
    }

    fn grow(&mut self) {
        let mut new_char_count = self.char_count.clone();
        let mut new_pair_count = self.pair_count.clone();
        for (&pair, &count) in &self.pair_count {
            if let Some(&to_insert) = self.insertion_map.get(&pair) {
                let first = pair[0];
                let second = pair[1];
                *new_char_count.entry(to_insert).or_default() += count;
                new_pair_count.entry(pair).and_modify(|x| *x -= count);
                *new_pair_count.entry([first, to_insert]).or_default() += count;
                *new_pair_count.entry([to_insert, second]).or_default() += count;
            }
        }

        self.pair_count = new_pair_count;
        self.char_count = new_char_count;
    }

    fn grow_repeat(&mut self, steps: u32) {
        for _ in 0..steps {
            self.grow()
        }
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let mut polymer = Polymer::new(&input);
    polymer.grow_repeat(10);
    let output_1 = polymer.get_difference();

    polymer.grow_repeat(30);
    let output_2 = polymer.get_difference();

    println!("part 1: {output_1} part 2: {output_2}")
}
