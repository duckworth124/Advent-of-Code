use itertools::Itertools;
use nom::{bytes, multi, IResult, Parser};

use crate::sensor::Sensor;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Sensors(pub Vec<Sensor>);

impl Sensors {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        multi::separated_list0(bytes::complete::tag("\n\n"), Sensor::parse)
            .map(Self)
            .parse(input)
    }

    pub fn count_unique(&self) -> usize {
        self.0
            .iter()
            .cloned()
            .flat_map(|s| s.beacons.transform(s.transformation.unwrap()))
            .unique()
            .count()
    }

    pub fn max_manhattan_distance(&self) -> u32 {
        self.0
            .iter()
            .map(|s| s.get_position().unwrap())
            .permutations(2)
            .map(|v| v[0].manhattan_distance(v[1]))
            .max()
            .unwrap()
    }
}
