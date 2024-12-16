use std::fs::read_to_string;

use regex::Regex;

struct Population([u64; 9]);

impl Population {
    fn new(input: &str) -> Self {
        let mut population = [0; 9];
        for timer in get_all_numbers(input) {
            population[timer] += 1;
        }

        Population(population)
    }

    fn count(&self) -> u64 {
        self.0.iter().sum()
    }

    fn simulate(&mut self, time: u32) {
        for _ in 0..time {
            self.0.rotate_left(1);
            self.0[6] += self.0[8]
        }
    }
}

fn get_all_numbers(line: &str) -> Vec<usize> {
    let num_pat = Regex::new(r"\d+").unwrap();
    num_pat
        .find_iter(line)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let mut population = Population::new(&input);
    population.simulate(80);
    let output_1 = population.count();
    population.simulate(256 - 80);
    let output_2 = population.count();
    println!("part 1: {output_1} part 2: {output_2}")
}
