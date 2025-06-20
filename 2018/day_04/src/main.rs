use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

fn guard_id(line: &str) -> Option<u32> {
    Some(line.split_once('#')?.1.split_once(' ')?.0.parse().unwrap())
}

fn get_minute(line: &str) -> usize {
    line.split_once(':')
        .unwrap()
        .1
        .split_once(']')
        .unwrap()
        .0
        .parse()
        .unwrap()
}

fn solve(input: &str) -> (u32, u32) {
    let mut sleep_times: HashMap<u32, [u32; 60]> = HashMap::new();
    let mut current_guard = 0;
    for line in input.lines().sorted_unstable() {
        if let Some(guard) = guard_id(line) {
            current_guard = guard;
            continue;
        }

        let minute = get_minute(line);
        let sleep_time = sleep_times.entry(current_guard).or_insert([0; 60]);
        if line.contains("falls asleep") {
            sleep_time[minute..].iter_mut().for_each(|x| *x += 1);
        } else {
            sleep_time[minute..].iter_mut().for_each(|x| *x -= 1);
        }
    }

    let &sleepiest_guard = sleep_times
        .iter()
        .map(|(i, a)| (i, a.iter().sum::<u32>()))
        .max_by_key(|(_, s)| *s)
        .unwrap()
        .0;

    let sleepiest_minute = sleep_times[&sleepiest_guard]
        .iter()
        .enumerate()
        .max_by_key(|(_, x)| *x)
        .unwrap()
        .0 as u32;

    let output_1 = sleepiest_guard * sleepiest_minute;

    let (sleepiest_guard, sleepiest_minute, _) = sleep_times
        .iter()
        .flat_map(|(&g, a)| a.iter().enumerate().map(move |(i, c)| (g, i, c)))
        .max_by_key(|(_, _, x)| **x)
        .unwrap();
    let output_2 = sleepiest_guard * sleepiest_minute as u32;

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
