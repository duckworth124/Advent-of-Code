#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn rotate_left(self) -> Self {
        self.rotate_right().rotate_right().rotate_right()
    }

    fn to_coords(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

fn solve(instructions: &[(char, u32)]) -> u32 {
    instructions
        .iter()
        .scan(Direction::Right, |direction, (c, d)| todo!());
}

fn main() {
    println!("Hello, world!");
}
