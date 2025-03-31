use std::fs::read_to_string;

fn fuel(mass: u32) -> u32 {
    if mass == 0 {
        0
    } else {
        let f = (mass / 3).saturating_sub(2);
        f + fuel(f)
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let masses: Vec<u32> = input.lines().map(|s| s.parse().unwrap()).collect();

    let output_1: u32 = masses.iter().map(|m| m / 3 - 2).sum();
    let output_2: u32 = masses.iter().copied().map(fuel).sum();

    println!("part 1: {output_1} part 2: {output_2}")
}
