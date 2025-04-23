use std::{collections::HashMap, fs::read_to_string, iter};

use itertools::iproduct;

fn split(grid: &[Vec<bool>], size: usize) -> Vec<Vec<Vec<Vec<bool>>>> {
    let mut output =
        vec![vec![vec![vec![false; size]; size]; grid[0].len() / size]; grid.len() / size];
    for (i, j, k, l) in iproduct!(
        0..grid.len() / size,
        0..grid[0].len() / size,
        0..size,
        0..size
    ) {
        output[i][j][k][l] = grid[size * i + k][size * j + l]
    }

    output
}

fn unsplit(grid: Vec<Vec<Vec<Vec<bool>>>>) -> Vec<Vec<bool>> {
    let size = grid[0][0].len();
    let height = grid.len() * size;
    let width = grid[0].len() * size;
    let mut output = vec![vec![false; width]; height];
    for (i, j, k, l) in iproduct!(0..grid.len(), 0..grid[0].len(), 0..size, 0..size) {
        output[size * i + k][size * j + l] = grid[i][j][k][l]
    }

    output
}

fn rotate_left(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let size = grid.len();
    let mut output = vec![vec![false; size]; size];
    for (i, j) in iproduct!(0..size, 0..size) {
        output[size - j - 1][i] = grid[i][j]
    }

    output
}

fn all_rotations(grid: &[Vec<bool>]) -> Vec<Vec<Vec<bool>>> {
    let flipped: Vec<Vec<bool>> = grid.iter().cloned().rev().collect();
    [
        iter::successors(Some(grid.to_vec()), |g| Some(rotate_left(g)))
            .take(4)
            .collect::<Vec<_>>(),
        iter::successors(Some(flipped), |g| Some(rotate_left(g)))
            .take(4)
            .collect::<Vec<_>>(),
    ]
    .into_iter()
    .flatten()
    .collect()
}
fn enhance_grid(
    grid: &[Vec<bool>],
    rules: &HashMap<Vec<Vec<bool>>, Vec<Vec<bool>>>,
) -> Vec<Vec<bool>> {
    all_rotations(grid)
        .into_iter()
        .find_map(|g| rules.get(&g))
        .map_or(grid, |v| v)
        .to_vec()
}

fn update(grid: &[Vec<bool>], rules: &HashMap<Vec<Vec<bool>>, Vec<Vec<bool>>>) -> Vec<Vec<bool>> {
    let height = grid.len();
    let size = if height % 2 == 0 { 2 } else { 3 };
    let enhanced: Vec<Vec<Vec<Vec<bool>>>> = split(grid, size)
        .into_iter()
        .map(|v| v.into_iter().map(|v| enhance_grid(&v, rules)).collect())
        .collect();

    unsplit(enhanced)
}

fn solve(input: &str) -> (u32, u32) {
    let rules: HashMap<Vec<Vec<bool>>, Vec<Vec<bool>>> = input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(" => ").unwrap();
            let l: Vec<Vec<bool>> = l
                .split('/')
                .map(|s| {
                    s.chars()
                        .map(|c| match c {
                            '.' => false,
                            '#' => true,
                            _ => panic!("unrecognized char: {c}"),
                        })
                        .collect()
                })
                .collect();

            let r: Vec<Vec<bool>> = r
                .split('/')
                .map(|s| {
                    s.chars()
                        .map(|c| match c {
                            '.' => false,
                            '#' => true,
                            _ => panic!("unrecognized char: {c}"),
                        })
                        .collect()
                })
                .collect();

            (l, r)
        })
        .collect();

    let mut grid = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];

    for _ in 0..5 {
        grid = update(&grid, &rules)
    }

    let output_1 = grid.iter().flatten().map(|&b| b as u32).sum();

    for _ in 0..13 {
        grid = update(&grid, &rules)
    }

    let output_2 = grid.iter().flatten().map(|&b| b as u32).sum();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
