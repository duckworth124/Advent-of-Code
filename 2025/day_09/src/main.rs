use itertools::Itertools;
use std::{
    fs::read_to_string,
    ops::{Add, Sub},
    time::Instant,
};

#[derive(Clone, Copy)]
struct Vec2D([i64; 2]);

impl Vec2D {
    const fn orientation(self, other: Self) -> i64 {
        (self.0[0] * other.0[1] - self.0[1] * other.0[0]).signum()
    }

    const fn area(self, other: Self) -> u64 {
        (self.0[0].abs_diff(other.0[0]) + 1) * (self.0[1].abs_diff(other.0[1]) + 1)
    }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.0[0] + rhs.0[0];
        let y = self.0[1] + rhs.0[1];
        Self([x, y])
    }
}

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.0[0] - rhs.0[0];
        let y = self.0[1] - rhs.0[1];
        Self([x, y])
    }
}

fn is_valid_rectangle(
    reds: &[Vec2D],
    orientations: &[i64],
    corners: [usize; 2],
    overall_orientation: i64,
) -> bool {
    if orientations[corners[0] + 1..corners[1]]
        .iter()
        .copied()
        .all(|o| o != overall_orientation)
    {
        return false;
    }
    if orientations[..corners[0]]
        .iter()
        .chain(&orientations[corners[1] + 1..])
        .copied()
        .all(|o| o != overall_orientation)
    {
        return false;
    }

    let corners = corners.map(|i| reds[i]);
    let (min_x, max_x) = corners
        .map(|v| v.0[0])
        .into_iter()
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = corners
        .map(|v| v.0[1])
        .into_iter()
        .minmax()
        .into_option()
        .unwrap();

    if reds
        .iter()
        .copied()
        .circular_tuple_windows()
        .map(|(l, r)| ((l.0[0] + r.0[0]) / 2, (l.0[1] + r.0[1]) / 2))
        .any(|(x, y)| x > min_x && x < max_x && y > min_y && y < max_y)
    {
        return false;
    }

    true
}

fn solve(input: &str) -> (u64, u64) {
    let reds: Vec<Vec2D> = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(l, r)| [l.parse().unwrap(), r.parse().unwrap()])
        .map(Vec2D)
        .collect();

    let orientations: Vec<i64> = reds[reds.len() - 1..]
        .iter()
        .chain(&reds)
        .copied()
        .tuple_windows()
        .map(|(l, r)| r - l)
        .collect_vec()
        .into_iter()
        .circular_tuple_windows()
        .map(|(l, r)| l.orientation(r))
        .collect();

    let output_1 = reds
        .iter()
        .copied()
        .tuple_combinations()
        .map(|(l, r)| l.area(r))
        .max()
        .unwrap();

    let overall_orientation: i64 = orientations.iter().sum::<i64>().signum();

    let output_2 = (0..reds.len())
        .array_combinations()
        .filter(|&c| is_valid_rectangle(&reds, &orientations, c, overall_orientation))
        .map(|c| c.map(|i| reds[i]))
        .map(|[l, r]| l.area(r))
        .max()
        .unwrap();

    (output_1, output_2)
}
fn main() {
    let time = Instant::now();
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
