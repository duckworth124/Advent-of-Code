use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn bearing(self) -> Bearing {
        let quadrant = self.quadrant();
        let mut numerator = self.y;
        let mut denominator = self.x;
        if denominator < 0 {
            denominator *= -1;
            numerator *= -1;
        }
        let max = numerator.abs().max(denominator.abs());
        for i in (2..=max).rev() {
            if denominator % i == 0 && numerator % i == 0 {
                denominator /= i;
                numerator /= i;
            }
        }
        Bearing {
            quadrant,
            numerator,
            denominator,
        }
    }

    fn bearing_from(self, other: Self) -> Bearing {
        let x = self.x - other.x;
        let y = self.y - other.y;
        Self { x, y }.bearing()
    }

    const fn quadrant(self) -> Quadrant {
        if self.x >= 0 {
            if self.y <= 0 {
                return Quadrant::TopRight;
            }
            return Quadrant::BottomRight;
        }
        if self.y >= 0 {
            return Quadrant::BottlomLeft;
        }
        Quadrant::TopLeft
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Quadrant {
    TopRight,
    BottomRight,
    BottlomLeft,
    TopLeft,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Bearing {
    quadrant: Quadrant,
    numerator: i32,
    denominator: i32,
}

impl Ord for Bearing {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.quadrant
            .cmp(&other.quadrant)
            .then_with(|| match self.quadrant {
                Quadrant::TopRight => {
                    (other.numerator * self.denominator).cmp(&(self.numerator * other.denominator))
                }
                Quadrant::BottomRight => {
                    (other.numerator * self.denominator).cmp(&(self.numerator * other.denominator))
                }
                Quadrant::BottlomLeft => {
                    (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
                }
                Quadrant::TopLeft => {
                    (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
                }
            })
    }
}

impl PartialOrd for Bearing {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn count_visible(asteroids: &HashSet<Position>, asteroid: Position) -> usize {
    asteroids
        .iter()
        .filter(|&&a| a != asteroid)
        .map(|p| p.bearing_from(asteroid))
        .unique()
        .count()
}

fn get_200th(asteroids: &HashSet<Position>, centre: Position) -> Position {
    let mut bearings: HashMap<Bearing, Vec<Position>> = HashMap::new();
    asteroids
        .iter()
        .copied()
        .filter(|a| *a != centre)
        .for_each(|a| bearings.entry(a.bearing_from(centre)).or_default().push(a));
    bearings
        .values_mut()
        .for_each(|v| v.sort_by_key(|a| (a.x.abs(), a.y.abs())));
    bearings
        .into_iter()
        .flat_map(|(b, v)| v.into_iter().enumerate().map(move |(i, p)| (i, b, p)))
        .sorted_by_key(|&(i, p, _)| (i, p))
        //.inspect(|x| println!("{:?}", x.2))
        .nth(199)
        .unwrap()
        .2
}

fn solve(input: &str) -> (usize, i32) {
    let asteroids: HashSet<Position> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .filter(|(_, _, c)| *c == '#')
        .map(|(x, y, _)| (x as i32, y as i32))
        .map(|(x, y)| Position { x, y })
        .collect();

    let (centre, n) = asteroids
        .iter()
        .map(|&a| (a, count_visible(&asteroids, a)))
        .max_by_key(|&(_, n)| n)
        .unwrap();

    let output_1 = n;
    let asteroid_200 = get_200th(&asteroids, centre);
    let output_2 = 100 * asteroid_200.x + asteroid_200.y;

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
