use std::{fs::read_to_string, iter};

#[derive(Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(self, direction: Direction) -> Self {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let (x, y) = (self.x + dx, self.y + dy);
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
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
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn rotate_left(self) -> Self {
        self.rotate_right().rotate_right().rotate_right()
    }
}

#[derive(Clone, Copy, Debug)]
struct Agent {
    position: Position,
    direction: Direction,
}

impl Agent {
    fn step(self, grid: &[Vec<char>]) -> Option<Self> {
        let current = grid[self.position.y as usize][self.position.x as usize];
        let new_state = if current == '+' {
            [self.direction.rotate_left(), self.direction.rotate_right()]
                .into_iter()
                .map(|d| (self.position.step(d), d))
                .map(|(position, direction)| Self {
                    position,
                    direction,
                })
                .find(|a| get(grid, a.position).is_some_and(|c| c != ' '))
                .unwrap()
        } else {
            let position = self.position.step(self.direction);
            Self { position, ..self }
        };

        Some(new_state).filter(|a| get(grid, a.position).is_some_and(|c| c != ' '))
    }
}

fn get(grid: &[Vec<char>], position: Position) -> Option<char> {
    let y: usize = position.y.try_into().ok()?;
    let x: usize = position.x.try_into().ok()?;
    grid.get(y)?.get(x).copied()
}

fn solve(input: &str) -> (String, usize) {
    let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let start_x = (0..grid[0].len()).find(|&x| grid[0][x] == '|').unwrap();
    let agent = Agent {
        position: Position {
            x: start_x as i32,
            y: 0,
        },
        direction: Direction::Down,
    };

    let path: Vec<Position> = iter::successors(Some(agent), |a| a.step(&grid))
        .map(|a| a.position)
        .collect();

    let output_1 = path
        .iter()
        .map(|&p| get(&grid, p))
        .flatten()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    let output_2 = path.len();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
