use itertools::Itertools;
use std::{
    fs::{read_to_string, File},
    io::Write,
};
use winnow::{
    ascii::dec_int,
    combinator::{repeat, repeat_till},
    error::ErrorKind,
    token::any,
    Parser,
};

fn simulate(robots: &[[i32; 4]], width: i32, height: i32, steps: i32) -> Vec<[i32; 2]> {
    robots
        .iter()
        .copied()
        .map(|[x, y, dx, dy]: [i32; 4]| [x + steps * dx, y + steps * dy])
        .map(|[x, y]| [x.rem_euclid(width), y.rem_euclid(height)])
        .collect()
}

fn process_input(mut input: &str) -> Vec<[i32; 4]> {
    repeat(
        0..,
        repeat(
            4,
            repeat_till(0.., any::<&str, ErrorKind>, dec_int).map(|((), i): ((), i32)| i),
        )
        .try_map(|v: Vec<_>| <[i32; 4]>::try_from(v)),
    )
    .parse_next(&mut input)
    .unwrap()
}

fn render(positions: &[[usize; 2]], width: usize, height: usize) -> String {
    let mut grid = vec![vec!['.'; width]; height];
    for [x, y] in positions {
        grid[*y][*x] = '#';
    }

    grid.join(&'\n').into_iter().collect()
}

fn is_probably_a_christmas_tree(positions: &[[usize; 2]]) -> bool {
    positions.iter().all_unique()
}

fn solve(path: &str) -> u32 {
    let input = read_to_string(path).unwrap();
    let (width, height) = if path == "input" { (101, 103) } else { (11, 7) };
    let robots = process_input(&input);
    let output = simulate(&robots, width, height, 100)
        .into_iter()
        .filter(|[x, y]| *x != width / 2 && *y != height / 2)
        .map(|[x, y]| [x < width / 2, y < height / 2])
        .map(|[x, y]| [x as usize, y as usize])
        .map(|[x, y]| x * 2 + y)
        .fold([0; 4], |mut acc, x| {
            acc[x] += 1;
            acc
        })
        .into_iter()
        .product();

    let mut file = File::create("output").unwrap();
    for i in 0.. {
        let positions: Vec<_> = simulate(&robots, width, height, i)
            .into_iter()
            .map(|x| x.map(|i| i as usize))
            .collect();

        if is_probably_a_christmas_tree(&positions) {
            writeln!(
                file,
                "{i}:\n{}\n",
                render(&positions, width as usize, height as usize)
            )
            .unwrap();
            break;
        }
    }

    output
}

fn main() {
    let output = solve("input");
    println!("part 1: {output}")
}
