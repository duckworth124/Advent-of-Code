use itertools::iproduct;
use std::{fs::read_to_string, time::Instant};

struct Tile {
    occupied: bool,
    surrounding_occupied: usize,
}

fn solve(input: &str) -> (usize, usize) {
    let mut grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|_| Tile {
                    occupied: false,
                    surrounding_occupied: 0,
                })
                .collect()
        })
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if c == '@' {
                grid[y][x].occupied = true;
                (y.saturating_sub(1)..=(y + 1).min(height - 1)).for_each(|y| {
                    for x in x.saturating_sub(1)..=(x + 1).min(width - 1) {
                        grid[y][x].surrounding_occupied += 1
                    }
                });
            }
        }
    }
    let output_1 = grid
        .iter()
        .flatten()
        .filter(|t| t.occupied && t.surrounding_occupied <= 4)
        .count();

    let mut output_2 = 0;
    let mut frontier: Vec<(usize, usize)> = iproduct!(0..width, 0..height).collect();
    while let Some((x, y)) = frontier.pop() {
        if !grid[y][x].occupied {
            continue;
        }
        if grid[y][x].surrounding_occupied >= 5 {
            continue;
        }
        grid[y][x].occupied = false;
        output_2 += 1;
        (y.saturating_sub(1)..=(y + 1).min(height - 1)).for_each(|y| {
            for x in x.saturating_sub(1)..=(x + 1).min(width - 1) {
                grid[y][x].surrounding_occupied -= 1;
                frontier.push((x, y));
            }
        });
    }

    (output_1, output_2)
}

fn main() {
    let time = Instant::now();
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
