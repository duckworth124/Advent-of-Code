use std::{cmp::Reverse, fs::read_to_string};

fn count_possible(containers: &[u32], target: u32) -> u32 {
    if containers.is_empty() {
        return (target == 0) as u32;
    }

    let mut output = count_possible(&containers[1..], target);
    if containers[0] <= target {
        output += count_possible(&containers[1..], target - containers[0])
    }

    output
}

fn min_number_of_containers(containers: &[u32], target: u32) -> Option<Reverse<u32>> {
    if containers.is_empty() {
        if target == 0 {
            return Some(Reverse(0));
        } else {
            return None;
        }
    }

    let mut output = min_number_of_containers(&containers[1..], target);
    if containers[0] <= target {
        output = output.max(
            min_number_of_containers(&containers[1..], target - containers[0])
                .map(|n| Reverse(n.0 + 1)),
        )
    }

    output
}

fn count_possible_constrained(containers: &[u32], target: u32, to_use: u32) -> u32 {
    if containers.is_empty() {
        if target == 0 && to_use == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    let mut output = count_possible_constrained(&containers[1..], target, to_use);
    if containers[0] <= target && to_use > 0 {
        output += count_possible_constrained(&containers[1..], target - containers[0], to_use - 1)
    }

    output
}

fn main() {
    let input = read_to_string("input").unwrap();
    let containers: Vec<u32> = input.lines().map(|s| s.parse().unwrap()).collect();
    let output_1 = count_possible(&containers, 150);
    let min = min_number_of_containers(&containers, 150).unwrap().0;
    let output_2 = count_possible_constrained(&containers, 150, min);
    println!("part 1: {output_1} part 2: {output_2}")
}
