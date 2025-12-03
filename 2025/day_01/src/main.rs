use std::{fs::read_to_string, time::Instant};

fn solve(input: &str) -> (i32, i32) {
    let mut position = 50;
    let mut output_1 = 0;
    let mut output_2 = 0;
    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();
        let change = if direction == 'L' { -1 } else { 1 } * distance;

        let was_0_before = position == 0;
        position += change;
        if position > 99 {
            output_2 += position / 100
        }
        if position <= 0 {
            output_2 += 1 - (position) / 100 - was_0_before as i32;
        }
        position = position.rem_euclid(100);
        if position == 0 {
            output_1 += 1;
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
