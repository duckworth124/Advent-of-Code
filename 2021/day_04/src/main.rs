use std::fs::read_to_string;

use regex::Regex;

#[derive(Debug, Clone)]
struct DrawNumbers(Vec<u32>);

impl DrawNumbers {
    fn new(input: &str) -> Self {
        let numbers = get_all_numbers(input);
        DrawNumbers(numbers)
    }
}

impl IntoIterator for DrawNumbers {
    type Item = u32;
    type IntoIter = <std::vec::Vec<u32> as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone)]
struct Grid {
    numbers: Vec<Vec<u32>>,
    marked: [[bool; 5]; 5],
    complete: bool,
}

impl Grid {
    fn new(input: &str) -> Self {
        let numbers = input.lines().map(get_all_numbers).collect();
        Grid {
            numbers,
            marked: [[false; 5]; 5],
            complete: false,
        }
    }

    fn is_bingo(&self, x: usize, y: usize) -> bool {
        let row = self.marked[y];
        let column = self.marked.map(|r| r[x]);
        row.into_iter().all(|b| b) || column.into_iter().all(|b| b)
    }

    fn score(&self, x: usize, y: usize) -> Option<u32> {
        if !self.is_bingo(x, y) {
            return None;
        }

        let sum_of_unmarked: u32 = self
            .numbers
            .iter()
            .zip(self.marked)
            .flat_map(|(r_1, r_2)| r_1.iter().zip(r_2))
            .filter(|(_, marked)| !*marked)
            .map(|(n, _)| n)
            .sum();

        Some(sum_of_unmarked * self.numbers[y][x])
    }

    fn find(&self, number: u32) -> Option<(usize, usize)> {
        let y = self.numbers.iter().position(|v| v.contains(&number))?;
        let x = self.numbers[y].iter().position(|n| *n == number)?;
        Some((x, y))
    }

    fn mark(&mut self, number: u32) -> Option<u32> {
        if self.complete {
            return None;
        }

        let (x, y) = self.find(number)?;
        self.marked[y][x] = true;
        let output = self.score(x, y);
        self.complete = output.is_some();
        output
    }
}

#[derive(Clone)]
struct Grids(Vec<Grid>);

impl Grids {
    fn new(input: &str) -> Self {
        let grids = input.split("\n\n").skip(1).map(Grid::new).collect();
        Grids(grids)
    }

    fn mark(&mut self, number: u32) -> Vec<u32> {
        self.0
            .iter_mut()
            .filter_map(move |g| g.mark(number))
            .collect()
    }
}

fn get_all_numbers(line: &str) -> Vec<u32> {
    let num_pat = Regex::new(r"\d+").unwrap();
    num_pat
        .find_iter(line.lines().next().unwrap())
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

fn get_first_score(mut grids: Grids, draw_numbers: DrawNumbers) -> u32 {
    draw_numbers
        .into_iter()
        .flat_map(|n| grids.mark(n))
        .next()
        .unwrap()
}

fn get_last_score(mut grids: Grids, draw_numbers: DrawNumbers) -> u32 {
    draw_numbers
        .into_iter()
        .flat_map(|n| grids.mark(n))
        .last()
        .unwrap()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let draw_numbers = DrawNumbers::new(&input);
    let grids = Grids::new(&input);

    let output_1 = get_first_score(grids.clone(), draw_numbers.clone());
    let output_2 = get_last_score(grids, draw_numbers);
    println!("part 1: {output_1} part 2: {output_2}")
}
