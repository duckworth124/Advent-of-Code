use itertools::Itertools;
use regex::Regex;
use std::{cmp::Ordering, collections::HashSet, fs::read_to_string};

enum SimulationResult {
    Hit { max_height: i32 },
    Miss,
}

#[derive(Clone, Copy)]
struct PositionRange {
    min: i32,
    max: i32,
}

impl PositionRange {
    fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }

    fn contains(&self, point: i32) -> bool {
        self.min <= point && point <= self.max
    }
}

fn all_numbers(line: &str) -> Vec<i32> {
    let num_pat = Regex::new(r"-?\d+").unwrap();
    num_pat
        .find_iter(line)
        .map(|m| m.as_str().parse().unwrap())
        .collect()
}

fn get_max_travel_time(max_initial_y_velocity: i32, vertial_range: PositionRange) -> i32 {
    let mut current_position = 0;
    let mut current_velocity = max_initial_y_velocity;
    let mut travel_time = 0;
    loop {
        travel_time += 1;
        current_position += current_velocity;
        current_velocity -= 1;

        if current_position < vertial_range.min && current_velocity <= 0 {
            break travel_time;
        }
    }
}

fn get_hit_times(
    initial_x_velocity: i32,
    horizontal_range: PositionRange,
    max_travel_time: i32,
) -> Vec<i32> {
    let mut current_position = 0;
    let mut current_velocity = initial_x_velocity;
    let mut output = vec![];
    let mut travel_time = 0;
    loop {
        travel_time += 1;

        if travel_time > max_travel_time {
            break output;
        }

        current_position += current_velocity;

        match current_velocity.cmp(&0) {
            Ordering::Less => current_velocity += 1,
            Ordering::Greater => current_velocity -= 1,
            Ordering::Equal => {}
        }

        if horizontal_range.contains(current_position) {
            output.push(travel_time);
        }
    }
}

fn get_possible_hit_times(horizontal_range: PositionRange, max_travel_time: i32) -> HashSet<i32> {
    let max_initial_x_velocity = horizontal_range.max;
    (0..=max_initial_x_velocity)
        .flat_map(|x| get_hit_times(x, horizontal_range, max_travel_time))
        .collect()
}

fn simulate_flight(
    initial_y_velocity: i32,
    vertial_range: PositionRange,
    possible_hit_times: &HashSet<i32>,
) -> SimulationResult {
    let mut current_position = 0;
    let mut current_velocity = initial_y_velocity;
    for travel_time in 1..=*possible_hit_times.iter().max().unwrap() {
        current_position += current_velocity;
        current_velocity -= 1;

        if vertial_range.contains(current_position) && possible_hit_times.contains(&travel_time) {
            let max_height = initial_y_velocity * (initial_y_velocity + 1) / 2;
            return SimulationResult::Hit { max_height };
        }
    }

    SimulationResult::Miss
}

fn try_initial_velocity(
    initial_x_velocity: i32,
    initial_y_velocity: i32,
    vertical_range: PositionRange,
    horizontal_range: PositionRange,
    max_travel_time: i32,
) -> bool {
    let possible_hit_times: HashSet<_> =
        get_hit_times(initial_x_velocity, horizontal_range, max_travel_time)
            .into_iter()
            .collect();

    if possible_hit_times.is_empty() {
        return false;
    }

    let simulation_result =
        simulate_flight(initial_y_velocity, vertical_range, &possible_hit_times);
    matches!(simulation_result, SimulationResult::Hit { max_height: _ })
}

fn solve(path: &str) -> (i32, usize) {
    let input = read_to_string(path).unwrap();
    let nums = all_numbers(&input);
    let horizontal_range = PositionRange::new(nums[0], nums[1]);
    let vertical_range = PositionRange::new(nums[2], nums[3]);
    let max_initial_y_velocity = -vertical_range.min - 1;
    let min_initial_y_velocity = vertical_range.min;
    let max_initial_x_velocity = horizontal_range.max;

    let max_travel_time = get_max_travel_time(max_initial_y_velocity, vertical_range);
    let possible_hit_times = get_possible_hit_times(horizontal_range, max_travel_time);

    let max_height = (0..=max_initial_y_velocity)
        .rev()
        .map(|initial_y_velocity| {
            simulate_flight(initial_y_velocity, vertical_range, &possible_hit_times)
        })
        .filter_map(|r| match r {
            SimulationResult::Miss => None,
            SimulationResult::Hit { max_height } => Some(max_height),
        })
        .next()
        .unwrap();

    let total_valid_velocities = (0..=max_initial_x_velocity)
        .cartesian_product(min_initial_y_velocity..=max_initial_y_velocity)
        .filter(|&(x, y)| {
            try_initial_velocity(x, y, vertical_range, horizontal_range, max_travel_time)
        })
        .count();

    (max_height, total_valid_velocities)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}

#[test]
fn practice() {
    let (output_1, output_2) = solve("practice");
    assert_eq!(output_1, 45);
    assert_eq!(output_2, 112)
}
