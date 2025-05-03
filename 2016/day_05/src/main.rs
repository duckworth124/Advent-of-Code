use md5::compute;
use std::{
    fs::read_to_string,
    io::{self, Write},
};

fn generate_password_1(input: &str) -> String {
    (0..)
        .map(|i| format!("{input}{i}"))
        .map(compute)
        .map(|d| format!("{d:x}"))
        .filter(|s| &s[..5] == "00000")
        .take(8)
        .map(|s| s.chars().nth(5).unwrap())
        .collect()
}

fn clear_screen() {
    print!("\x1b[2J\r");
}

fn generate_password_2(input: &str) -> String {
    let mut output = ['-'; 8];
    print!("{}", output.iter().collect::<String>());
    for i in 0.. {
        let data = format!("{input}{i}");
        let s = format!("{:x}", compute(data));
        let s = match s.strip_prefix("00000") {
            Some(s) => s,
            None => continue,
        };
        let index = s.chars().next().unwrap();
        if !index.is_ascii_digit() {
            continue;
        }

        let index = index as usize - '0' as usize;
        if index >= 8 {
            continue;
        }
        if output[index] != '-' {
            continue;
        }

        let c = s.chars().nth(1).unwrap();

        output[index] = c;
        clear_screen();
        print!("{}", output.iter().collect::<String>());
        io::stdout().flush().unwrap();
        if output.iter().all(|&c| c != '-') {
            break;
        }
    }
    println!();

    output.iter().collect()
}

fn solve(input: &str) -> (String, String) {
    let input = input.trim();
    let output_1 = generate_password_1(input);
    let output_2 = generate_password_2(input);
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
