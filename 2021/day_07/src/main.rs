use std::fs::read_to_string;

use regex::Regex;

struct Crabs(Vec<i32>);

impl Crabs {
    fn new(input: &str) -> Self {
        let mut crabs = get_all_numbers(input);
        crabs.sort();
        Crabs(crabs)
    }

    fn get_fuel_linear_cost(&self, crab_position: i32) -> u32 {
        self.0.iter().map(|c| c.abs_diff(crab_position)).sum()
    }

    fn get_fuel_quadratic_cost(&self, crab_position: i32) -> u32 {
        self.0
            .iter()
            .map(|c| {
                let distance = c.abs_diff(crab_position);
                distance * (distance + 1) / 2
            })
            .sum()
    }

    fn get_min_fuel_linear_cost(&self) -> u32 {
        let median_index = (self.0.len() - 1) / 2;
        let median_crab = self.0[median_index];
        self.get_fuel_linear_cost(median_crab)
    }

    fn is_strictly_increasing_at_point(&self, crab_position: i32) -> bool {
        self.get_fuel_quadratic_cost(crab_position)
            < self.get_fuel_quadratic_cost(crab_position + 1)
    }

    fn get_min_fuel_quadratic_cost(&self) -> u32 {
        let lower = *self.0.first().unwrap();
        let upper = *self.0.last().unwrap();
        let optimal = (lower..=upper)
            .find(|p| self.is_strictly_increasing_at_point(*p))
            .unwrap();

        self.get_fuel_quadratic_cost(optimal)
    }
}

fn get_all_numbers(input: &str) -> Vec<i32> {
    let num_pat = Regex::new(r"\d+").unwrap();
    num_pat
        .find_iter(input)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let crabs = Crabs::new(&input);
    let output_1 = crabs.get_min_fuel_linear_cost();
    let output_2 = crabs.get_min_fuel_quadratic_cost();
    println!("part 1: {output_1} part 2: {output_2}")
}
