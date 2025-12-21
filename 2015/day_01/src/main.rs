use std::{fs::read_to_string, time::Instant};

fn solve(path: &str) -> (i32, usize) {
    let input = read_to_string(path).unwrap();
    let mut output_1 = 0;
    let mut output_2 = None;
    let mut steps = 0;
    for c in input.trim().chars() {
        steps += 1;
        if c == '(' {
            output_1 += 1;
        } else {
            output_1 -= 1;
        }
        if output_1 < 0 {
            output_2 = output_2.or(Some(steps))
        }
    }
    (output_1, output_2.unwrap())
}

fn main() {
    let time = Instant::now();
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
