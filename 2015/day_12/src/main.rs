use std::{fs::read_to_string, time::Instant};

fn parse_int(input: &mut &str) -> i32 {
    let neg = input.starts_with('-');
    if neg {
        *input = &input[1..];
    };
    let mut output = 0;
    while let Some(c) = input.chars().next().filter(|c| c.is_ascii_digit()) {
        output *= 10;
        output += c as i32 - '0' as i32;
        *input = &input[1..];
    }
    if neg {
        output *= -1
    }
    output
}

fn parse_string(input: &mut &str) -> bool {
    *input = &input[1..];
    if input.starts_with("red\"") {
        *input = &input[4..];
        return true;
    }
    let p = input.find('"').unwrap();
    *input = &input[p + 1..];
    false
}

fn parse_array(input: &mut &str, ignore_red: bool) -> i32 {
    *input = &input[1..];
    let mut total = 0;
    loop {
        let c = input.chars().next().unwrap();
        total += match c {
            '-' | '0'..='9' => parse_int(input),
            '"' => {
                parse_string(input);
                0
            }
            '[' => parse_array(input, ignore_red),

            '{' => parse_object(input, ignore_red),

            _ => unreachable!("unexpected char: {c:?}"),
        };
        if input.starts_with(']') {
            *input = &input[1..];
            break total;
        }
        *input = &input[1..];
    }
}

fn parse_object(input: &mut &str, ignore_red: bool) -> i32 {
    *input = &input[1..];
    let mut total = 0;
    let mut contains_red = false;
    loop {
        parse_string(input);
        *input = &input[1..];
        let c = input.chars().next().unwrap();
        total += match c {
            '-' | '0'..='9' => parse_int(input),
            '"' => {
                if parse_string(input) {
                    contains_red = true;
                };
                0
            }
            '[' => parse_array(input, ignore_red),
            '{' => parse_object(input, ignore_red),
            _ => unreachable!("unexpected char: {c:?}"),
        };
        if input.starts_with('}') {
            if contains_red && ignore_red {
                total = 0;
            }
            *input = &input[1..];
            break total;
        }
        *input = &input[1..]
    }
}

fn parse(mut input: &str, ignore_red: bool) -> i32 {
    let c = input.chars().next().unwrap();
    match c {
        '-' | '0'..='9' => parse_int(&mut input),
        '"' => 0,
        '[' => parse_array(&mut input, ignore_red),
        '{' => parse_object(&mut input, ignore_red),
        _ => unreachable!("unexpected char: {c:?}"),
    }
}

fn solve(input: &str) -> (i32, i32) {
    (parse(input, false), parse(input, true))
}

fn main() {
    let time = Instant::now();
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(input.trim());
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
