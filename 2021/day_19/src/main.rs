use std::{fs::read_to_string, vec};

use day_19::sensors::Sensors;
use itertools::Itertools;

fn solve(path: &str) -> (usize, u32) {
    let input = read_to_string(path).unwrap();
    let mut sensors = Sensors::parse(&input).unwrap().1;

    sensors.0.get_mut(0).unwrap().transformation = Some(vec![]);
    while sensors
        .0
        .iter()
        .any(|s| s.transformation.is_none())
    {
        for [i, j] in (0..sensors.0.len())
            .permutations(2)
            .map(|v| [v[0], v[1]])
        {
            let sensor_i = &sensors.0[i];
            let sensor_j = &sensors.0[j];
            if sensor_i.transformation.is_some() {
                continue;
            }
            let sensor_j_transformation = match &sensor_j.transformation {
                Some(t) => t.clone(),
                None => continue,
            };
            if let Some((rotation, translation)) = sensor_i.find_relative_transformation(sensor_j) {
                sensors.0[i].transformation =
                    Some([vec![(rotation, translation)], sensor_j_transformation].concat())
            }
        }
    }

    (sensors.count_unique(), sensors.max_manhattan_distance())
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
