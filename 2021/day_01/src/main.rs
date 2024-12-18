use std::fs::read_to_string;

fn get_increases(v: Vec<u32>) -> usize {
    v.iter()
        .zip(v.iter().skip(1))
        .filter(|(prev, next)| prev < next)
        .count()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let depths: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();
    let windows_of_3: Vec<u32> = depths.windows(3).map(|v| v.iter().sum()).collect();

    let output_1 = get_increases(depths);
    let output_2 = get_increases(windows_of_3);
    println!("part 1: {output_1} part 2: {output_2}")
}
