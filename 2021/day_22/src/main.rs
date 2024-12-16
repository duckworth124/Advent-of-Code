use nom::{
    branch,
    bytes::complete::tag,
    character::complete::{char, i32, newline},
    combinator::value,
    multi,
    sequence::{preceded, terminated},
    IResult, Parser,
};
use std::{collections::HashMap, fs::read_to_string, ops::RangeInclusive};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Cuboid {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
    z_range: RangeInclusive<i32>,
}

impl Cuboid {
    fn parse(input: &str) -> IResult<&str, Self> {
        let range = |c: char| {
            preceded(char(c).and(char('=')), terminated(i32, tag("..")).and(i32))
                .map(|(min, max)| min..=max)
        };

        terminated(range('x'), char(','))
            .and(terminated(range('y'), char(',')))
            .and(range('z'))
            .map(|((x_range, y_range), z_range)| Self {
                x_range,
                y_range,
                z_range,
            })
            .parse(input)
    }

    fn clamp_abs_50(self) -> Self {
        let x_min = *self.x_range.start();
        let x_max = *self.x_range.end();
        let y_min = *self.y_range.start();
        let y_max = *self.y_range.end();
        let z_min = *self.z_range.start();
        let z_max = *self.z_range.end();

        let x_range = x_min.max(-50)..=x_max.min(50);
        let y_range = y_min.max(-50)..=y_max.min(50);
        let z_range = z_min.max(-50)..=z_max.min(50);

        Self {
            x_range,
            y_range,
            z_range,
        }
    }

    fn volume(&self) -> usize {
        let x_len = (self.x_range.end() - self.x_range.start() + 1).max(0) as i64;
        let y_len = (self.y_range.end() - self.y_range.start() + 1).max(0) as i64;
        let z_len = (self.z_range.end() - self.z_range.start() + 1).max(0) as i64;

        (x_len * y_len * z_len) as usize
    }

    fn is_empty(&self) -> bool {
        self.volume() == 0
    }

    fn intersection(&self, other: &Self) -> Self {
        let min_x = *self.x_range.start().max(other.x_range.start());
        let max_x = *self.x_range.end().min(other.x_range.end());
        let min_y = *self.y_range.start().max(other.y_range.start());
        let max_y = *self.y_range.end().min(other.y_range.end());
        let min_z = *self.z_range.start().max(other.z_range.start());
        let max_z = *self.z_range.end().min(other.z_range.end());

        let x_range = min_x..=max_x;
        let y_range = min_y..=max_y;
        let z_range = min_z..=max_z;

        Self {
            x_range,
            y_range,
            z_range,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Instruction {
    cuboid: Cuboid,
    command: Command,
}

impl Instruction {
    fn parse(line: &str) -> IResult<&str, Self> {
        terminated(Command::parse, char(' '))
            .and(Cuboid::parse)
            .map(|(command, cuboid)| Self { command, cuboid })
            .parse(line)
    }

    fn clamp_abs_50(self) -> Self {
        Self {
            cuboid: self.cuboid.clamp_abs_50(),
            ..self
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Instructions(Vec<Instruction>);

impl Instructions {
    fn parse(input: &str) -> IResult<&str, Self> {
        multi::separated_list0(newline, Instruction::parse)
            .map(Self)
            .parse(input)
    }

    fn apply(&self) -> Set {
        self.0
            .iter()
            .fold(Set::Empty, |acc, instruction| match instruction.command {
                Command::On => Set::Union(
                    Box::new(acc),
                    Box::new(Set::Cuboid(instruction.cuboid.clone())),
                ),
                Command::Off => Set::Difference(
                    Box::new(acc),
                    Box::new(Set::Cuboid(instruction.cuboid.clone())),
                ),
            })
    }

    fn clamped(&self) -> Self {
        Self(
            self.0
                .iter()
                .cloned()
                .map(|i| i.clamp_abs_50())
                .collect(),
        )
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Command {
    On,
    Off,
}

impl Command {
    fn parse(input: &str) -> IResult<&str, Self> {
        let on = value(Self::On, tag("on"));
        let off = value(Self::Off, tag("off"));

        branch::alt((on, off))(input)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Set {
    Cuboid(Cuboid),
    Union(Box<Self>, Box<Self>),
    Difference(Box<Self>, Box<Self>),
    Empty,
}

impl Set {
    fn intersection(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Empty, _) => Self::Empty,
            (Self::Cuboid(first), Self::Cuboid(second)) => {
                let intersection = first.intersection(second);
                if intersection.is_empty() {
                    Self::Empty
                } else {
                    Self::Cuboid(intersection)
                }
            }
            (Self::Union(u1, u2), other_set) => Self::Union(
                Box::new(u1.intersection(other_set)),
                Box::new(u2.intersection(other_set)),
            ),
            (Self::Difference(d1, d2), other_set) => {
                Self::Difference(Box::new(d1.intersection(other_set)), d2.clone())
            }

            (_, _) => other.intersection(self),
        }
    }

    fn volume(&self, cache: &mut HashMap<Self, usize>) -> usize {
        if let Some(v) = cache.get(self) {
            return *v;
        }

        let output = match self {
            Self::Empty => 0,
            Self::Cuboid(cuboid) => cuboid.volume(),
            Self::Union(first, second) => {
                first.volume(cache) + second.volume(cache)
                    - first.intersection(second).volume(cache)
            }

            Self::Difference(first, second) => {
                first.volume(cache) - first.intersection(second).volume(cache)
            }
        };

        cache.insert(self.clone(), output);
        output
    }
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let instructions = Instructions::parse(&input).unwrap().1;

    let output_1 = instructions
        .clamped()
        .apply()
        .volume(&mut HashMap::new());
    let output_2 = instructions.apply().volume(&mut HashMap::new());

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
