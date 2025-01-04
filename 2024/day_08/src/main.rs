use itertools::{iterate, Itertools};
use std::{
    collections::HashMap,
    fs::read_to_string,
    ops::{Add, Neg, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (x, y) = (self.x + rhs.x, self.y + rhs.y);
        Self { x, y }
    }
}

impl Neg for Position {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let (x, y) = (-self.x, -self.y);
        Self { x, y }
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

fn process_input(input: &str) -> HashMap<char, Vec<Position>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, r)| r.char_indices().map(move |(x, c)| (x, y, c)))
        .filter(|(_, _, c)| *c != '.')
        .map(|(x, y, c)| (x as i32, y as i32, c))
        .map(|(x, y, c)| (c, Position { x, y }))
        .sorted_unstable_by_key(|(c, _)| *c)
        .chunk_by(|(c, _)| *c)
        .into_iter()
        .map(|(c, p)| (c, p.map(|x| x.1).collect_vec()))
        .collect()
}

fn get_antinodes(p1: Position, p2: Position) -> [Position; 2] {
    [p1 + p1 - p2, p2 + p2 - p1]
}

fn get_more_antinodes(p1: Position, p2: Position, width: i32, height: i32) -> Vec<Position> {
    iterate(p1, |p| *p + p1 - p2)
        .take_while(|p| is_in_bounds(*p, width, height))
        .chain(iterate(p2, |p| *p + p2 - p1).take_while(|p| is_in_bounds(*p, width, height)))
        .collect_vec()
}

fn is_in_bounds(p: Position, width: i32, height: i32) -> bool {
    (0..width).contains(&p.x) && (0..height).contains(&p.y)
}

fn count_unique_antinodes(
    positions: &HashMap<char, Vec<Position>>,
    width: i32,
    height: i32,
    many_antinodes: bool,
) -> usize {
    positions
        .values()
        .flat_map(|v| {
            v.iter()
                .permutations(2)
                .map(|v| (v[0], v[1]))
                .flat_map(|(&p1, &p2)| {
                    if many_antinodes {
                        get_more_antinodes(p1, p2, width, height)
                    } else {
                        get_antinodes(p1, p2).to_vec()
                    }
                })
        })
        .unique()
        .filter(|p| is_in_bounds(*p, width, height))
        .count()
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let positions = process_input(&input);
    let output_1 = count_unique_antinodes(&positions, width, height, false);
    let output_2 = count_unique_antinodes(&positions, width, height, true);
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
