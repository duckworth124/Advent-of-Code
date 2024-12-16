use std::fs::read_to_string;

#[derive(Clone)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl Direction {
    fn new(input: &str) -> Self {
        match input {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("invalid character"),
        }
    }
}

#[derive(Clone)]
struct Instruction {
    distance: i32,
    direction: Direction,
}

impl Instruction {
    fn new(input: &str) -> Self {
        let direction = input.chars().take_while(|c| *c != ' ').collect::<String>();
        let direction = Direction::new(&direction);
        let distance = input
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();

        Self {
            direction,
            distance,
        }
    }
}

struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }
    fn apply_instruction(&self, instruction: Instruction, with_aim: bool) -> Self {
        if with_aim {
            match instruction.direction {
                Direction::Up => {
                    let aim = self.aim - instruction.distance;
                    Position { aim, ..*self }
                }

                Direction::Down => {
                    let aim = self.aim + instruction.distance;
                    Position { aim, ..*self }
                }

                Direction::Forward => {
                    let x = self.x + instruction.distance;
                    let y = self.y + self.aim * instruction.distance;
                    Position { x, y, ..*self }
                }
            }
        } else {
            let (dx, dy) = match instruction.direction {
                Direction::Forward => (instruction.distance, 0),
                Direction::Up => (0, -instruction.distance),
                Direction::Down => (0, instruction.distance),
            };

            let (x, y) = (self.x + dx, self.y + dy);
            Position { x, y, ..*self }
        }
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let instructions: Vec<_> = input.lines().map(Instruction::new).collect();

    let mut current_position = Position::new();
    for instruction in instructions.clone() {
        current_position = current_position.apply_instruction(instruction, false);
    }
    let output_1 = current_position.x * current_position.y;

    let mut current_position = Position::new();
    for instruction in instructions {
        current_position = current_position.apply_instruction(instruction, true);
    }
    let output_2 = current_position.x * current_position.y;
    println!("part 1: {output_1} part 2: {output_2}")
}
