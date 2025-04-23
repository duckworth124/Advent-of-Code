use std::{
    collections::HashMap,
    fs::read_to_string,
    ops::{Add, AddAssign},
};

use itertools::Itertools;
use winnow::{
    Parser,
    ascii::dec_int,
    combinator::{repeat, repeat_till},
    token::any,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Vec3d {
    x: i64,
    y: i64,
    z: i64,
}

impl Add for Vec3d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (x, y, z) = (self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
        Self { x, y, z }
    }
}

impl AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

#[derive(Clone, Copy, Debug)]
struct Particle {
    position: Vec3d,
    velocity: Vec3d,
    acceleration: Vec3d,
}

impl Particle {
    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }

    fn will_never_collide(self, other: Self) -> bool {
        if (other.position.x - self.position.x) * (other.velocity.x - self.velocity.x) < 0 {
            return false;
        }

        if (other.acceleration.x - self.acceleration.x) * (other.velocity.x - self.velocity.x) < 0 {
            return false;
        }

        true
    }
}

fn all_numbers(mut input: &str) -> Vec<i64> {
    repeat(
        0..,
        repeat_till(0.., any::<&str, ()>, dec_int).map(|((), x): ((), i64)| x),
    )
    .parse_next(&mut input)
    .unwrap()
}

fn solve(input: &str) -> (usize, usize) {
    let particles: Vec<Vec<i64>> = input.lines().map(|mut l| all_numbers(&mut l)).collect();

    let mut particle_scores: Vec<(usize, (i64, i64, i64))> = particles
        .iter()
        .map(|v| {
            let mut v = v.clone();
            for i in 0..3 {
                if v[i + 6] < 0 {
                    for j in 0..3 {
                        v[i + 3 * j] *= -1;
                    }
                }
            }

            v
        })
        .map(|v| (v[6] + v[7] + v[8], v[3] + v[4] + v[5], v[0] + v[1] + v[2]))
        .enumerate()
        .collect();

    particle_scores.sort_unstable_by_key(|(_, x)| *x);
    let output_1 = particle_scores[0].0;

    let mut particles: Vec<Particle> = particles
        .into_iter()
        .map(|v| Particle {
            position: Vec3d {
                x: v[0],
                y: v[1],
                z: v[2],
            },
            velocity: Vec3d {
                x: v[3],
                y: v[4],
                z: v[5],
            },
            acceleration: Vec3d {
                x: v[6],
                y: v[7],
                z: v[8],
            },
        })
        .collect();

    'outer: loop {
        for p in particles.iter_mut() {
            p.update();
        }
        particles = particles
            .into_iter()
            .fold(
                HashMap::new(),
                |mut acc: HashMap<Vec3d, Vec<Particle>>, p: Particle| {
                    acc.entry(p.position).or_default().push(p);
                    acc
                },
            )
            .into_values()
            .filter(|v| v.len() == 1)
            .flatten()
            .collect();

        for (p1, p2) in particles
            .iter()
            .copied()
            .permutations(2)
            .map(|v| (v[0], v[1]))
        {
            if !p1.will_never_collide(p2) {
                continue 'outer;
            }
        }

        break;
    }

    let output_2 = particles.len();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
