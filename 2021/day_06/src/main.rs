use std::fs::read_to_string;

struct Population([u64; 9]);

impl Population {
    fn new(input: &str) -> Self {
        let mut population = [0; 9];
        for timer in input.trim().split(',').map(|s| s.parse::<usize>().unwrap()) {
            population[timer] += 1;
        }

        Self(population)
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

fn main() {
    let input = read_to_string("input").unwrap();
    let mut population = Population::new(&input);
    population.simulate(80);
    let output_1 = population.count();
    population.simulate(256 - 80);
    let output_2 = population.count();
    println!("part 1: {output_1} part 2: {output_2}")
}
