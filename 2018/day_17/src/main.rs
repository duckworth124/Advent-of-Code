use itertools::Itertools;
use std::{collections::HashSet, fmt::Display, fs::read_to_string};
use winnow::{
    Parser, Result,
    ascii::dec_int,
    combinator::{alt, preceded, separated_pair},
};

struct State {
    occupied: HashSet<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
    y_range: (i32, i32),
    seen: HashSet<(i32, i32)>,
}

impl State {
    fn is_blocked_below(&self, position: (i32, i32)) -> bool {
        let position = (position.0, position.1 + 1);
        self.occupied.contains(&position)
    }

    fn is_blocked(&self, position: (i32, i32)) -> bool {
        self.occupied.contains(&position)
    }

    fn spawn_water(&mut self, start: (i32, i32)) {
        let mut current = start;
        if !self.seen.insert(start) {
            return;
        }
        loop {
            while !self.is_blocked_below(current) {
                if current.1 > self.y_range.1 {
                    return;
                }
                if self.y_range.0 <= current.1 && current.1 <= self.y_range.1 {
                    self.visited.insert(current);
                }
                current.1 += 1;
            }
            self.visited.insert(current);

            let mut left = current;
            let mut is_left_blocked = false;
            loop {
                left.0 -= 1;
                if !self.is_blocked_below(left) {
                    self.spawn_water(left);
                    break;
                }
                if self.is_blocked(left) {
                    is_left_blocked = true;
                    break;
                }
                self.visited.insert(left);
            }

            let mut right = current;
            let mut is_right_blocked = false;
            loop {
                right.0 += 1;
                if !self.is_blocked_below(right) {
                    self.spawn_water(right);
                    break;
                }
                if self.is_blocked(right) {
                    is_right_blocked = true;
                    break;
                }
                self.visited.insert(right);
            }
            if is_left_blocked && is_right_blocked {
                let y = current.1;
                for x in (left.0 + 1)..=(right.0 - 1) {
                    self.occupied.insert((x, y));
                }
                current.1 -= 1;
                continue;
            }
            break;
        }
    }

    fn retained(&self) -> usize {
        self.visited.intersection(&self.occupied).count()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_range = self
            .occupied
            .iter()
            .copied()
            .chain(self.visited.iter().copied())
            .map(|x| x.0)
            .minmax()
            .into_option()
            .unwrap();
        for y in self.y_range.0..=self.y_range.1 {
            for x in x_range.0..=x_range.1 {
                let is_visited = self.visited.contains(&(x, y));
                let is_occupied = self.occupied.contains(&(x, y));
                let c = if is_visited && is_occupied {
                    '~'
                } else if is_visited {
                    '|'
                } else if is_occupied {
                    '#'
                } else {
                    '.'
                };
                write!(f, "{c}")?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct VerticalRange {
    x: i32,
    y_range: (i32, i32),
}

impl VerticalRange {
    fn coordinates(self) -> Vec<(i32, i32)> {
        (self.y_range.0..=self.y_range.1)
            .map(move |y| (self.x, y))
            .collect()
    }

    fn parse(input: &mut &str) -> Result<Self> {
        let x: i32 = preceded("x=", dec_int).parse_next(input)?;
        let y_range: (i32, i32) =
            preceded(", y=", separated_pair(dec_int, "..", dec_int)).parse_next(input)?;
        Ok(Self { x, y_range })
    }
}

struct HorizontalRange {
    y: i32,
    x_range: (i32, i32),
}

impl HorizontalRange {
    fn coordinates(self) -> Vec<(i32, i32)> {
        (self.x_range.0..=self.x_range.1)
            .map(move |x| (x, self.y))
            .collect()
    }

    fn parse(input: &mut &str) -> Result<Self> {
        let y: i32 = preceded("y=", dec_int).parse_next(input)?;
        let x_range: (i32, i32) =
            preceded(", x=", separated_pair(dec_int, "..", dec_int)).parse_next(input)?;
        Ok(Self { y, x_range })
    }
}

enum Range {
    Vertical(VerticalRange),
    Horizontal(HorizontalRange),
}

impl Range {
    fn coordinates(self) -> Vec<(i32, i32)> {
        match self {
            Self::Vertical(vertical_range) => vertical_range.coordinates(),
            Self::Horizontal(horizontal_range) => horizontal_range.coordinates(),
        }
    }

    fn parse(input: &mut &str) -> Result<Self> {
        alt((
            HorizontalRange::parse.map(Self::Horizontal),
            VerticalRange::parse.map(Self::Vertical),
        ))
        .parse_next(input)
    }
}

fn solve(input: &str) -> (usize, usize) {
    let occupied: HashSet<(i32, i32)> = input
        .lines()
        .flat_map(|s| Range::parse.parse(s).unwrap().coordinates())
        .collect();
    let y_range = occupied
        .iter()
        .copied()
        .map(|x| x.1)
        .minmax()
        .into_option()
        .unwrap();

    let mut state = State {
        occupied,
        visited: HashSet::new(),
        y_range,
        seen: HashSet::new(),
    };

    state.spawn_water((500, 0));

    let output_1 = state.visited.len();
    let output_2 = state.retained();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
