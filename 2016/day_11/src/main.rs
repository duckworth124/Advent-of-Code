use std::collections::{HashSet, VecDeque};

use itertools::{Itertools, chain};
use winnow::{
    Parser, Result,
    ascii::alpha0,
    combinator::{alt, delimited},
};

#[derive(Clone, Debug)]
struct State {
    current_floor: usize,
    floors: [Vec<Object>; 4],
    holding: (Object, Option<Object>),
}

impl State {
    fn min_steps(self) -> Option<u32> {
        let mut frontier = VecDeque::from([(self, 0)]);
        loop {
            let (current_state, steps) = frontier.pop_front()?;
            if !current_state.is_valid() {
                continue;
            }
            if current_state.is_goal() {
                return Some(steps);
            }
            frontier.extend(
                current_state
                    .get_possible()
                    .into_iter()
                    .map(|s| (s, steps + 1)),
            );
        }
    }

    fn is_goal(&self) -> bool {
        self.current_floor == 0 && self.floors[..3].iter().all(|f| f.is_empty())
    }

    fn is_valid(&self) -> bool {
        (0..4).map(|floor| self.get_objects(floor)).all(|objects| {
            let (microchips, generators): (HashSet<&String>, HashSet<&String>) = objects
                .iter()
                .fold(Default::default(), |(mut microchips, mut generators), o| {
                    match o {
                        Object::Microchip(s) => microchips.insert(s),
                        Object::Generator(s) => generators.insert(s),
                    };
                    (microchips, generators)
                });
            generators.is_empty() || microchips.is_subset(&generators)
        })
    }

    fn get_possible(&self) -> Vec<Self> {
        let objects = self.get_objects(self.current_floor);
        objects
            .iter()
            .map(|&o| (o.clone(), None))
            .chain(
                objects
                    .iter()
                    .permutations(2)
                    .map(|v| ((*v[0]).to_owned(), Some(v[1].to_owned()))),
            )
            .map(|(o1, o2)| {
                let new_objects: Vec<Object> = objects
                    .iter()
                    .filter(|&&o| *o != o1)
                    .filter(|o| o2 != Some(o))
                    .cloned()
                    .cloned()
                    .collect();
                let mut new_state = self.clone();
                new_state.floors[self.current_floor] = new_objects;
                new_state
            })
            .flat_map(|s| match self.current_floor {
                0 => vec![Self {
                    current_floor: 1,
                    ..s
                }],
                1 | 2 => vec![
                    Self {
                        current_floor: s.current_floor + 1,
                        ..s.clone()
                    },
                    Self {
                        current_floor: s.current_floor - 1,
                        ..s
                    },
                ],
                3 => vec![Self {
                    current_floor: 2,
                    ..s
                }],
                _ => unreachable!(),
            })
            .collect()
    }

    fn get_objects(&self, floor: usize) -> Vec<&Object> {
        let mut objects: Vec<&Object> = self.floors[floor].iter().collect();
        if self.current_floor == floor {
            let (o1, o2) = &self.holding;
            objects.extend(chain!([o1], o2));
        }
        objects
    }

    fn parse_floor(input: &mut &str) -> Result<Vec<Object>> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Object {
    Microchip(String),
    Generator(String),
}

impl Object {
    fn parse_microchip(input: &mut &str) -> Result<Self> {
        delimited("a ", alpha0, "-compatible microchip")
            .map(|s: &str| Self::Microchip(s.to_string()))
            .parse_next(input)
    }

    fn parse_generator(input: &mut &str) -> Result<Self> {
        delimited("a ", alpha0, " generator")
            .map(|s: &str| Self::Generator(s.to_string()))
            .parse_next(input)
    }

    fn parse(input: &mut &str) -> Result<Self> {
        alt((Self::parse_microchip, Self::parse_generator)).parse_next(input)
    }
}

fn solve(input: &str) -> u32 {
    todo!()
}

fn main() {
    println!("Hello, world!");
}
