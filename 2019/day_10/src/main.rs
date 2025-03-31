use std::{collections::HashSet, fs::read_to_string, iter};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn get_blocked(self, other: Self) -> impl Iterator<Item = Self> {
        let (mut dx, mut dy) = (other.x - self.x, other.y - self.y);
        let max = dx.abs().max(dy.abs());
        for i in (2..=max).rev() {
            if dx % i == 0 && dy % i == 0 {
                dx /= i;
                dy /= i;
            }
        }
        iter::successors(Some(other), move |p| {
            Some(Position {
                x: p.x + dx,
                y: p.y + dy,
            })
        })
        .skip(1)
    }
}

fn count_visible(asteroids: &HashSet<Position>, asteroid: Position) -> usize {
    let min_x = asteroids.iter().map(|p| p.x).min().unwrap();
    let min_y = asteroids.iter().map(|p| p.y).min().unwrap();
    let max_x = asteroids.iter().map(|p| p.x).max().unwrap();
    let max_y = asteroids.iter().map(|p| p.y).max().unwrap();
    let mut visible: HashSet<Position> = asteroids.clone();
    visible.remove(&asteroid);
    for &other in asteroids {
        if other == asteroid {
            continue;
        }
        for blocked in asteroid
            .get_blocked(other)
            .take_while(|p| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y)
        {
            visible.remove(&blocked);
        }
    }

    if asteroid.x == 5 && asteroid.y == 8 {
        dbg!(&visible);
    }

    visible.len()
}

fn solve(input: &str) -> usize {
    let asteroids: HashSet<Position> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .filter(|(_, _, c)| *c == '#')
        .map(|(x, y, _)| (x as i64, y as i64))
        .map(|(x, y)| Position { x, y })
        .collect();

    let (_, n) = asteroids
        .iter()
        .map(|&a| (a, count_visible(&asteroids, a)))
        //.inspect(|(a, n)| println!("{} {} {}", a.x, a.y, n))
        .max_by_key(|&(_, n)| n)
        .unwrap();

    n
}

fn main() {
    let input = read_to_string("input").unwrap();
    let output_1 = solve(&input);
    println!("part 1: {output_1}")
}
