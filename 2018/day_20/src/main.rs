use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};
use winnow::{
    Parser, Result,
    combinator::{alt, delimited, repeat, separated},
};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(input: &mut &str) -> Result<Self> {
        alt((
            'N'.value(Self::Up),
            'S'.value(Self::Down),
            'E'.value(Self::Right),
            'W'.value(Self::Left),
        ))
        .parse_next(input)
    }

    const fn to_coords(self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

enum RegexGroup {
    Single(Direction),
    Branch(Vec<Regex>),
}

impl RegexGroup {
    fn parse(input: &mut &str) -> Result<Self> {
        alt((
            Direction::parse.map(Self::Single),
            delimited('(', separated(1.., Regex::parse, '|'), ')').map(Self::Branch),
        ))
        .parse_next(input)
    }

    fn update(&self, map: &mut Map, start: (i32, i32)) -> HashSet<(i32, i32)> {
        match self {
            Self::Single(direction) => {
                let (dx, dy) = direction.to_coords();
                let (x, y) = start;
                let new = (x + dx, y + dy);
                map.add_connection(start, new);
                HashSet::from([new])
            }
            Self::Branch(items) => items.iter().flat_map(|r| r.update(map, start)).collect(),
        }
    }
}

struct Regex {
    groups: Vec<RegexGroup>,
}

impl Regex {
    fn parse(input: &mut &str) -> Result<Self> {
        repeat(0.., RegexGroup::parse)
            .map(|groups| Self { groups })
            .parse_next(input)
    }

    fn update(&self, map: &mut Map, start: (i32, i32)) -> HashSet<(i32, i32)> {
        let mut output = HashSet::from([start]);
        for group in &self.groups {
            output = output
                .into_iter()
                .flat_map(|p| group.update(map, p))
                .collect();
        }

        output
    }
}

struct Map(HashSet<((i32, i32), (i32, i32))>);

impl Map {
    fn add_connection(&mut self, first: (i32, i32), second: (i32, i32)) {
        self.0.insert((first, second));
        self.0.insert((second, first));
    }

    fn max_distance(&self) -> u32 {
        let mut frontier = VecDeque::from([((0, 0), 0)]);
        let mut max = 0;
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        while let Some((current, distance)) = frontier.pop_front() {
            if !seen.insert(current) {
                continue;
            }
            max = max.max(distance);
            let (x, y) = current;
            for next in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                if self.0.contains(&(current, next)) {
                    frontier.push_back((next, distance + 1));
                }
            }
        }
        max
    }

    fn count_far_rooms(&self) -> u32 {
        let mut frontier = VecDeque::from([((0, 0), 0)]);
        let mut count = 0;
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        while let Some((current, distance)) = frontier.pop_front() {
            if !seen.insert(current) {
                continue;
            }
            if distance >= 1000 {
                count += 1;
            }
            let (x, y) = current;
            for next in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                if self.0.contains(&(current, next)) {
                    frontier.push_back((next, distance + 1));
                }
            }
        }
        count
    }
}

fn solve(input: &str) -> (u32, u32) {
    let input = input
        .trim()
        .strip_prefix('^')
        .unwrap()
        .strip_suffix('$')
        .unwrap();
    let regex = Regex::parse.parse(input).unwrap();
    let mut map = Map(HashSet::new());
    regex.update(&mut map, (0, 0));
    let output_1 = map.max_distance();
    let output_2 = map.count_far_rooms();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
