use std::{collections::HashMap, fs::read_to_string};

use regex::Regex;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn l1_distance(&self, other: Position) -> u32 {
        self.x.abs_diff(other.x).max(self.y.abs_diff(other.y))
    }
}

#[derive(Clone)]
struct Line(Position, Position);

impl Line {
    fn new(line: &str) -> Self {
        let nums = get_all_numbers(line);
        let (x1, y1, x2, y2) = (nums[0], nums[1], nums[2], nums[3]);

        let p1 = Position { x: x1, y: y1 };
        let p2 = Position { x: x2, y: y2 };
        Line(p1, p2)
    }

    fn is_orthogonal(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }

    fn positions(&self) -> Vec<Position> {
        let min_x = self.0.x.min(self.1.x);
        let max_x = self.0.x.max(self.1.x);
        let min_y = self.0.y.min(self.1.y);
        let max_y = self.0.y.max(self.1.y);

        let top_left = Position { x: min_x, y: min_y };

        if top_left == self.0 || top_left == self.1 {
            (min_x..=max_x)
                .cycle()
                .zip((min_y..=max_y).cycle())
                .take(self.0.l1_distance(self.1) as usize + 1)
                .map(|(x, y)| Position::new(x, y))
                .collect()
        } else {
            (min_x..=max_x)
                .rev()
                .zip(min_y..=max_y)
                .map(|(x, y)| Position::new(x, y))
                .collect()
        }
    }
}

#[derive(Clone)]
struct Lines(Vec<Line>);

impl Lines {
    fn new(input: &str) -> Self {
        let lines = input.lines().map(Line::new).collect();
        Lines(lines)
    }
}

impl IntoIterator for Lines {
    type Item = Line;

    type IntoIter = <std::vec::Vec<Line> as std::iter::IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

struct OverlapCounts(HashMap<Position, u32>);

impl OverlapCounts {
    fn new(lines: Lines, orthogonal_only: bool) -> Self {
        let mut map = HashMap::new();
        for line in lines
            .into_iter()
            .filter(|line| !orthogonal_only || line.is_orthogonal())
        {
            for position in line.positions() {
                *map.entry(position).or_insert(0) += 1
            }
        }

        OverlapCounts(map)
    }

    fn count_more_than_2(&self) -> usize {
        self.0.iter().filter(|(_, c)| **c >= 2).count()
    }
}

fn get_all_numbers(line: &str) -> Vec<i32> {
    let num_pat = Regex::new(r"\d+").unwrap();
    num_pat
        .find_iter(line)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let lines = Lines::new(&input);
    let counts_orthogonal_only = OverlapCounts::new(lines.clone(), true);
    let counts_with_diagonal = OverlapCounts::new(lines, false);

    let output_1 = counts_orthogonal_only.count_more_than_2();
    let output_2 = counts_with_diagonal.count_more_than_2();
    println!("part 1: {output_1} part 2: {output_2}")
}
