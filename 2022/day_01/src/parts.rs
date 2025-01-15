fn process_input(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse().unwrap()).collect())
        .collect()
}

pub fn part_1(input: &str) -> i32 {
    let calories = process_input(input);
    calories.iter().map(|v| v.iter().sum()).max().unwrap()
}

const NUMBER_OF_ELVES: usize = 3;
pub fn part_2(input: &str) -> i32 {
    let mut calories: Vec<i32> = process_input(input)
        .iter()
        .map(|v| v.iter().sum())
        .collect();
    calories.sort_unstable();
    calories.iter().rev().take(NUMBER_OF_ELVES).sum()
}
