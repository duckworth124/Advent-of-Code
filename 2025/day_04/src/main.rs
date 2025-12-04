use itertools::{Itertools, iproduct};
use std::{fs::read_to_string, time::Instant};

fn can_be_removed(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut output = vec![];
    let height = grid.len();
    let width = grid[0].len();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '@'
                && iproduct!(
                    y.saturating_sub(1)..=(y + 1).min(height - 1),
                    x.saturating_sub(1)..=(x + 1).min(width - 1)
                )
                .unique()
                .map(|(y, x)| grid[y][x])
                .filter(|&c| c == '@')
                .count()
                    <= 4
            {
                output.push((x, y));
            }
        }
    }
    output
}

fn solve(input: &str) -> (usize, usize) {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut to_remove = can_be_removed(&grid);
    let output_1 = to_remove.len();
    let mut output_2 = 0;
    while !to_remove.is_empty() {
        output_2 += to_remove.len();
        for (x, y) in to_remove {
            grid[y][x] = '.';
        }
        to_remove = can_be_removed(&grid)
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
