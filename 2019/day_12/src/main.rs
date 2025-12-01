use std::{collections::HashMap, fs::read_to_string, ops::BitAnd};
use winnow::{
    Parser, Result,
    ascii::{dec_int, line_ending},
    combinator::{delimited, preceded, separated},
};

#[derive(Debug, Clone, Copy)]
struct Moon {
    position: [i32; 3],
    velocity: [i32; 3],
}

impl Moon {
    fn step(&mut self) {
        self.position
            .iter_mut()
            .zip(self.velocity)
            .for_each(|(p, v)| *p += v);
    }

    fn parse(input: &mut &str) -> Result<Self> {
        delimited(
            '<',
            (
                preceded("x=", dec_int),
                preceded(", y=", dec_int),
                preceded(", z=", dec_int),
            )
                .map(|x| Self {
                    position: x.into(),
                    velocity: [0; 3],
                }),
            '>',
        )
        .parse_next(input)
    }

    fn total_energy(&self) -> i32 {
        self.position.map(|x| x.abs()).iter().sum::<i32>()
            * self.velocity.map(|x| x.abs()).iter().sum::<i32>()
    }
}

#[derive(Clone, Copy)]
struct State {
    moons: [Moon; 4],
}

impl State {
    fn step(&mut self) {
        for i in 0..4 {
            for j in i + 1..4 {
                for k in 0..3 {
                    if self.moons[i].position[k] > self.moons[j].position[k] {
                        self.moons[i].velocity[k] -= 1;
                        self.moons[j].velocity[k] += 1;
                        continue;
                    }

                    if self.moons[i].position[k] < self.moons[j].position[k] {
                        self.moons[i].velocity[k] += 1;
                        self.moons[j].velocity[k] -= 1;
                        continue;
                    }
                }
            }
        }

        self.moons.iter_mut().for_each(|m| m.step());
    }

    fn total_energy(&self) -> i32 {
        self.moons.iter().map(|m| m.total_energy()).sum()
    }

    fn parse(input: &mut &str) -> Result<Self> {
        separated(4, Moon::parse, line_ending)
            .map(|moons: Vec<Moon>| Self {
                moons: moons.try_into().unwrap(),
            })
            .parse_next(input)
    }

    fn find_cycle(mut self, index: usize) -> Cycle {
        let mut seen = HashMap::new();
        let mut steps = 0;
        loop {
            self.step();
            if let Some(prev) = seen.insert(
                self.moons.map(|m| [m.position[index], m.velocity[index]]),
                steps,
            ) {
                let len = steps - prev;
                let start = prev;
                return Cycle { start, len };
            }
            steps += 1;
        }
    }
}

struct Cycle {
    start: u64,
    len: u64,
}

impl BitAnd for Cycle {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let start = self.start.max(rhs.start);
        let len = lcm(self.len, rhs.len);
        Self { start, len }
    }
}

fn gcd(x: u64, y: u64) -> u64 {
    let mut low = x.min(y);
    let mut high = x.max(y);
    while low > 0 {
        high %= low;
        std::mem::swap(&mut low, &mut high);
    }
    high
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

fn solve(input: &str) -> (i32, u64) {
    let state = State::parse.parse(input).unwrap();
    let mut current = state;
    for _ in 0..1000 {
        current.step();
    }
    let output_1 = current.total_energy();
    let x_cycle = state.find_cycle(0);
    let y_cycle = state.find_cycle(1);
    let z_cycle = state.find_cycle(2);
    let cycle = x_cycle & y_cycle & z_cycle;
    let output_2 = cycle.start + cycle.len;
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(input.trim());
    println!("part 1: {output_1} part 2: {output_2}")
}
