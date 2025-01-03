use itertools::Itertools;
use std::fs::read_to_string;

fn wrapping_paper((l, w, h): (u32, u32, u32)) -> u32 {
    let areas = [l * w, w * h, l * h];
    let min = areas[0].min(areas[1]).min(areas[2]);
    areas.into_iter().map(|a| a * 2).sum::<u32>() + min
}

fn ribbon((l, w, h): (u32, u32, u32)) -> u32 {
    let perimeters = [l + w, l + h, w + h];
    let min = perimeters[0].min(perimeters[1]).min(perimeters[2]);
    let volume = l * w * h;
    min * 2 + volume
}

fn solve(path: &str) -> (u32, u32) {
    let input = read_to_string(path).unwrap();
    let dimensions = input
        .lines()
        .map(|l| {
            l.split('x')
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let output_1 = dimensions.iter().copied().map(wrapping_paper).sum();
    let output_2 = dimensions.iter().copied().map(ribbon).sum();
    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
