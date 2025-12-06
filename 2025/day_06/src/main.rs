use std::{fs::read_to_string, time::Instant};

fn solve(input: &str) -> (u64, u64) {
    let breaks: Vec<usize> = (0..input.lines().next().unwrap().len())
        .filter(|&x| input.lines().all(|line| &line[x..=x] == " "))
        .collect();

    let grid: Vec<Vec<&str>> = input
        .lines()
        .map(|l| {
            breaks
                .windows(2)
                .map(|v| &l[v[0] + 1..v[1]])
                .chain([&l[..breaks[0]]])
                .chain([&l[breaks.last().unwrap() + 1..]])
                .collect()
        })
        .collect();

    let height = grid.len();
    let width = grid[0].len();
    let mut output_1 = 0;
    let mut output_2 = 0;
    for x in 0..width {
        let nums = (0..height - 1)
            .map(|y| grid[y][x])
            .map(|s| s.trim().parse::<u64>().unwrap());
        output_1 += if grid[height - 1][x].contains('+') {
            nums.sum::<u64>()
        } else {
            nums.product::<u64>()
        };

        let nums = (0..grid[0][x].len()).map(|x2| {
            (0..height - 1)
                .map(|y| &grid[y][x][x2..=x2])
                .skip_while(|&s| s == " ")
                .take_while(|&s| s != " ")
                .map(|s| s.chars().next().unwrap())
                .map(|s| s as u64 - '0' as u64)
                .fold(0, |acc, s| acc * 10 + s)
        });

        output_2 += if grid[height - 1][x].contains('+') {
            nums.sum::<u64>()
        } else {
            nums.product::<u64>()
        }
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
