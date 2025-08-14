use std::{collections::VecDeque, fs::read_to_string};

const fn josephus(mut n: usize) -> usize {
    let mut output = 1;
    while !n.is_power_of_two() {
        output += 2;
        n -= 1;
    }
    output
}

fn main() {
    let input = read_to_string("input").unwrap();
    let n = input.trim().parse().unwrap();
    let output_1 = josephus(n);

    let mut state = VecDeque::from_iter(1..=n);
    state.rotate_left(state.len() / 2);

    while (state.len()) > 1 {
        state.pop_front();
        if state.len() % 2 == 0 {
            state.rotate_left(1);
        }
    }

    let output_2 = state[0];

    println!("part 1: {output_1} part 2: {output_2}")
}
