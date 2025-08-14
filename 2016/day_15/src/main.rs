use std::{fs::read_to_string, mem::swap, ops::BitAnd};

#[derive(Clone, Copy)]
struct Constraint {
    divided_by: u32,
    remainder: u32,
}

impl Constraint {
    const fn apply(self, x: u32) -> bool {
        x % self.divided_by == self.remainder
    }
}

impl BitAnd for Constraint {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let divided_by = lcm(self.divided_by, rhs.divided_by);
        let remainder = (self.remainder..)
            .step_by(self.divided_by as usize)
            .find(|r| rhs.apply(*r))
            .unwrap();
        Self {
            divided_by,
            remainder,
        }
    }
}

fn lcm(x: u32, y: u32) -> u32 {
    x * y / gcd(x, y)
}

fn gcd(x: u32, y: u32) -> u32 {
    let mut max = x.max(y);
    let mut min = x.min(y);
    while min > 0 {
        max %= min;
        swap(&mut max, &mut min);
    }

    max
}

fn solve(input: &str) -> (u32, u32) {
    let mut discs: Vec<Constraint> = input
        .lines()
        .map(|l| {
            let disc_number: u32 = l
                .split_once('#')
                .unwrap()
                .1
                .split_once(' ')
                .unwrap()
                .0
                .parse()
                .unwrap();

            let divided_by: u32 = l
                .split_once("has ")
                .unwrap()
                .1
                .split_once(' ')
                .unwrap()
                .0
                .parse()
                .unwrap();

            let start: u32 = l
                .split_once("position ")
                .unwrap()
                .1
                .split_once('.')
                .unwrap()
                .0
                .parse()
                .unwrap();

            let remainder = (divided_by - disc_number % divided_by + divided_by
                - start % divided_by)
                % divided_by;

            Constraint {
                divided_by,
                remainder,
            }
        })
        .collect();

    let output_1 = discs
        .iter()
        .copied()
        .reduce(|acc, x| acc & x)
        .unwrap()
        .remainder;

    discs.push(Constraint {
        divided_by: 11,
        remainder: 4,
    });

    let output_2 = discs
        .iter()
        .copied()
        .reduce(|acc, x| acc & x)
        .unwrap()
        .remainder;

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
