use std::{collections::HashSet, fmt::Display, fs::read_to_string};
use winnow::{Parser, Result, ascii::dec_int, combinator::opt};

struct Light {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Light {
    const fn step(&mut self) {
        let (dx, dy) = self.velocity;
        self.position.0 += dx;
        self.position.1 += dy;
    }

    const fn step_back(&mut self) {
        let (dx, dy) = self.velocity;
        self.position.0 -= dx;
        self.position.1 -= dy;
    }

    fn parse(input: &mut &str) -> Result<Self> {
        "position=<".parse_next(input)?;
        opt(' ').parse_next(input)?;
        let x: i32 = dec_int(input)?;
        ','.parse_next(input)?;
        ' '.parse_next(input)?;
        opt(' ').parse_next(input)?;
        let y: i32 = dec_int(input)?;
        '>'.parse_next(input)?;
        ' '.parse_next(input)?;
        "velocity=<".parse_next(input)?;
        opt(' ').parse_next(input)?;
        let dx: i32 = dec_int(input)?;
        ','.parse_next(input)?;
        ' '.parse_next(input)?;
        opt(' ').parse_next(input)?;
        let dy: i32 = dec_int(input)?;
        '>'.parse_next(input)?;
        Ok(Self {
            position: (x, y),
            velocity: (dx, dy),
        })
    }
}

struct State {
    lights: Vec<Light>,
    time: i32,
}

impl State {
    fn step(&mut self) {
        self.lights.iter_mut().for_each(|l| l.step());
        self.time += 1;
    }

    fn step_back(&mut self) {
        self.lights.iter_mut().for_each(|l| l.step_back());
        self.time -= 1;
    }

    fn span(&self) -> (u32, u32) {
        let min_x = self.lights.iter().map(|l| l.position.0).min().unwrap();
        let max_x = self.lights.iter().map(|l| l.position.0).max().unwrap();
        let min_y = self.lights.iter().map(|l| l.position.1).min().unwrap();
        let max_y = self.lights.iter().map(|l| l.position.1).max().unwrap();

        (min_x.abs_diff(max_x), min_y.abs_diff(max_y))
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let span = self.span();
        let points: HashSet<(i32, i32)> = self.lights.iter().map(|l| l.position).collect();
        let min_x = self.lights.iter().map(|l| l.position.0).min().unwrap();
        let min_y = self.lights.iter().map(|l| l.position.1).min().unwrap();
        for y in 0..=span.1 as i32 {
            for x in 0..=span.0 as i32 {
                if points.contains(&(min_x + x, min_y + y)) {
                    write!(f, "#")?
                } else {
                    write!(f, ".")?
                }
            }

            writeln!(f)?
        }

        Ok(())
    }
}

fn solve(input: &str) -> (String, i32) {
    let lights = input
        .lines()
        .map(|l| Light::parse.parse(l).unwrap())
        .collect();
    let mut state = State { lights, time: 0 };

    let mut min_span = state.span();

    loop {
        state.step();
        let span = state.span();
        if span < min_span {
            min_span = span
        } else {
            break;
        }
    }
    state.step_back();

    (state.to_string(), state.time)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1:\n{output_1}part 2: {output_2}")
}
