use std::time::Instant;

use md5::compute;

fn solve(input: &str) -> (usize, usize) {
    let mut output_1 = None;
    for (i, s) in (0..)
        .map(|s| format!("{input}{s}"))
        .map(compute)
        .map(|s| format!("{s:x}"))
        .enumerate()
    {
        if s.starts_with("00000") {
            output_1 = output_1.or(Some(i));
        }
        if s.starts_with("000000") {
            return (output_1.unwrap(), i);
        }
    }
    unreachable!()
}

fn main() {
    let time = Instant::now();
    let (output_1, output_2) = solve("bgvyzdsv");
    println!("part 1: {output_1} part 1: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
