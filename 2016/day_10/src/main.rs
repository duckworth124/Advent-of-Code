use std::{fs::read_to_string, iter};
use winnow::{
    Parser, Result,
    ascii::dec_uint,
    combinator::{alt, preceded},
};

#[derive(Clone, Copy, Debug)]
struct Robot {
    chip_1: Option<u32>,
    chip_2: Option<u32>,
}

impl Robot {
    const fn give(&mut self, value: u32) {
        if self.chip_1.is_some() {
            self.chip_2 = Some(value)
        } else {
            self.chip_1 = Some(value)
        }
    }

    const fn new() -> Self {
        Self {
            chip_1: None,
            chip_2: None,
        }
    }
}

struct State {
    robots: Vec<Robot>,
    outputs: [u32; 3],
}

impl State {
    fn apply_rules(&mut self, mut rules: Vec<Rule>) {
        while !rules.is_empty() {
            for index in 0..rules.len() {
                if index >= rules.len() {
                    break;
                }
                let rule = rules[index];
                match rule {
                    Rule::ValueGoes { value, robot } => {
                        self.robots[robot].give(value);
                    }

                    Rule::RobotGives {
                        robot,
                        low_goes_to,
                        high_goes_to,
                    } => {
                        let robot = self.robots[robot];
                        let (chip_1, chip_2) = match (robot.chip_1, robot.chip_2) {
                            (Some(chip_1), Some(chip_2)) => (chip_1, chip_2),
                            _ => continue,
                        };
                        let low = chip_1.min(chip_2);
                        let high = chip_1.max(chip_2);
                        match low_goes_to {
                            Destination::Robot(robot) => {
                                self.robots[robot].give(low);
                            }
                            Destination::Output(o) => {
                                if o < 3 {
                                    self.outputs[o] = low
                                }
                            }
                        }
                        match high_goes_to {
                            Destination::Robot(robot) => {
                                self.robots[robot].give(high);
                            }
                            Destination::Output(o) => {
                                if o < 3 {
                                    self.outputs[o] = high
                                }
                            }
                        }
                    }
                }
                rules.swap_remove(index);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Rule {
    ValueGoes {
        value: u32,
        robot: usize,
    },

    RobotGives {
        robot: usize,
        low_goes_to: Destination,
        high_goes_to: Destination,
    },
}

impl Rule {
    fn parse_value_goes(input: &mut &str) -> Result<Self> {
        "value ".parse_next(input)?;
        let value: u32 = dec_uint(input)?;
        " goes to bot ".parse_next(input)?;
        let robot: usize = dec_uint(input)?;
        Ok(Self::ValueGoes { value, robot })
    }

    fn parse_robot_gives(input: &mut &str) -> Result<Self> {
        "bot ".parse_next(input)?;
        let robot: usize = dec_uint(input)?;
        " gives low to ".parse_next(input)?;
        let low_goes_to: Destination = Destination::parse(input)?;
        " and high to ".parse_next(input)?;
        let high_goes_to: Destination = Destination::parse(input)?;
        Ok(Self::RobotGives {
            robot,
            low_goes_to,
            high_goes_to,
        })
    }

    fn parse(input: &mut &str) -> Result<Self> {
        alt((Self::parse_robot_gives, Self::parse_value_goes)).parse_next(input)
    }

    fn get_robots_mentioned(self) -> Vec<usize> {
        match self {
            Self::ValueGoes { value: _, robot } => vec![robot],
            Self::RobotGives {
                robot,
                low_goes_to,
                high_goes_to,
            } => iter::once(robot)
                .chain(low_goes_to.get_robot_mentioned())
                .chain(high_goes_to.get_robot_mentioned())
                .collect(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Destination {
    Robot(usize),
    Output(usize),
}

impl Destination {
    fn parse_robot(input: &mut &str) -> Result<Self> {
        preceded("bot ", dec_uint)
            .map(Self::Robot)
            .parse_next(input)
    }

    fn parse_output(input: &mut &str) -> Result<Self> {
        preceded("output ", dec_uint)
            .map(Self::Output)
            .parse_next(input)
    }

    fn parse(input: &mut &str) -> Result<Self> {
        alt((Self::parse_output, Self::parse_robot)).parse_next(input)
    }

    const fn get_robot_mentioned(self) -> Option<usize> {
        match self {
            Self::Output(_) => None,
            Self::Robot(r) => Some(r),
        }
    }
}

fn solve(input: &str) -> (usize, u32) {
    let rules: Vec<Rule> = input
        .lines()
        .map(|mut l| Rule::parse(&mut l).unwrap())
        .collect();
    let max_robot_id = rules
        .iter()
        .flat_map(|r| r.get_robots_mentioned())
        .max()
        .unwrap();
    let num_robots = max_robot_id + 1;
    let mut state = State {
        robots: vec![Robot::new(); num_robots],
        outputs: [0; 3],
    };
    state.apply_rules(rules);
    let output_1 = state
        .robots
        .iter()
        .position(|r| {
            let low = r.chip_1.min(r.chip_2);
            let high = r.chip_1.max(r.chip_2);
            low == Some(17) && high == Some(61)
        })
        .unwrap();
    let output_2: u32 = state.outputs.iter().product();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
