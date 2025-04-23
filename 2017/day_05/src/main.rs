use std::fs::read_to_string;

fn solve(input: &str) -> (u32, u32) {
    let mut jumps: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();
    let jumps_clone = jumps.clone();
    let mut pc: i32 = 0;
    let mut steps = 0;
    let output_1 = loop {
        if pc < 0 || pc >= jumps.len() as i32 {
            break steps;
        }
        let jump = jumps.get_mut(pc as usize).unwrap();
        pc += *jump;
        *jump += 1;
        steps += 1;
    };

    jumps = jumps_clone;
    steps = 0;
    pc = 0;
    let output_2 = loop {
        if pc < 0 || pc >= jumps.len() as i32 {
            break steps;
        }
        let jump = jumps.get_mut(pc as usize).unwrap();
        pc += *jump;
        if *jump >= 3 {
            *jump -= 1;
        } else {
            *jump += 1;
        }
        steps += 1;
    };

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
