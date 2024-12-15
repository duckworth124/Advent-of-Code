use std::{fmt::Display, fs::read_to_string};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(self, direction: Direction) -> Self {
        let (x, y) = match direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };

        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct State {
    grid: Vec<Vec<Tile>>,
    robot: Position,
}

impl State {
    fn get(&self, position: Position) -> Option<Tile> {
        let (x, y): (usize, usize) = (position.x.try_into().ok()?, position.y.try_into().ok()?);
        self.grid.get(y).and_then(|r| r.get(x)).copied()
    }

    fn get_mut(&mut self, position: Position) -> Option<&mut Tile> {
        let (x, y): (usize, usize) = (position.x.try_into().ok()?, position.y.try_into().ok()?);
        self.grid.get_mut(y).and_then(|r| r.get_mut(x))
    }

    fn is_blocked(&self, position: Position) -> bool {
        self.get(position).unwrap_or(Tile::Wall) == Tile::Wall
    }

    fn is_empty(&self, position: Position) -> bool {
        self.get(position) == Some(Tile::Empty)
    }

    fn move_robot(&mut self, direction: Direction) {
        if !self.can_push(self.robot, direction) {
            return;
        }

        self.push(self.robot, direction);
        self.robot = self.robot.step(direction)
    }

    fn can_push(&self, position: Position, direction: Direction) -> bool {
        if self.is_blocked(position) {
            return false;
        }

        if self.is_empty(position) {
            return true;
        }

        let next_position = position.step(direction);

        if direction == Direction::Left || direction == Direction::Right {
            return self.can_push(next_position, direction);
        }

        match self.get(next_position) {
            Some(Tile::WideLeft) => {
                self.can_push(next_position, direction)
                    && self.can_push(next_position.step(Direction::Right), direction)
            }

            Some(Tile::WideRight) => {
                self.can_push(next_position, direction)
                    && self.can_push(next_position.step(Direction::Left), direction)
            }
            _ => self.can_push(next_position, direction),
        }
    }

    fn push(&mut self, position: Position, direction: Direction) {
        if self.is_empty(position) {
            return;
        }
        let next_position = position.step(direction);
        if direction == Direction::Up || direction == Direction::Down {
            match self.get(next_position) {
                Some(Tile::WideLeft) => {
                    self.push(next_position, direction);
                    self.push(next_position.step(Direction::Right), direction);
                }
                Some(Tile::WideRight) => {
                    self.push(next_position, direction);
                    self.push(next_position.step(Direction::Left), direction)
                }

                _ => self.push(next_position, direction),
            }
        } else {
            self.push(next_position, direction)
        }

        *self.get_mut(position.step(direction)).unwrap() = self.get(position).unwrap();
        *self.get_mut(position).unwrap() = Tile::Empty;
    }

    fn total_gps(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, t)| (x, y, t)))
            .filter(|(x, y, _)| *x as i32 != self.robot.x || *y as i32 != self.robot.y)
            .filter(|(_, _, t)| [Tile::Movable, Tile::WideLeft].contains(t))
            .map(|(x, y, _)| x + 100 * y)
            .sum()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, t) in row.iter().enumerate() {
                if x as i32 == self.robot.x && y as i32 == self.robot.y {
                    write!(f, "@")?;
                    continue;
                }

                let c = match t {
                    Tile::Wall => '#',
                    Tile::Empty => '.',
                    Tile::Movable => 'O',
                    Tile::WideLeft => '[',
                    Tile::WideRight => ']',
                };

                write!(f, "{c}")?
            }

            writeln!(f)?
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Movable,
    Wall,
    WideLeft,
    WideRight,
}

fn create_grid(input: &str, wide: bool) -> Vec<Vec<Tile>> {
    input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| {
                    if !wide {
                        match c {
                            '#' => vec![Tile::Wall],
                            '.' => vec![Tile::Empty],
                            'O' | '@' => vec![Tile::Movable],
                            _ => panic!("unrecognized character: {c}"),
                        }
                    } else {
                        match c {
                            '#' => vec![Tile::Wall; 2],
                            '.' => vec![Tile::Empty; 2],
                            'O' => vec![Tile::WideLeft, Tile::WideRight],
                            '@' => vec![Tile::Movable, Tile::Empty],
                            _ => panic!("unrecognised character: {c}"),
                        }
                    }
                })
                .collect()
        })
        .collect()
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let grid = create_grid(&input, false);

    let robot = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| l.char_indices().map(move |(x, c)| (x, y, c)))
        .map(|(x, y, c)| (x as i32, y as i32, c))
        .map(|(x, y, c)| (Position { x, y }, c))
        .find(|(_, c)| *c == '@')
        .unwrap()
        .0;

    let mut state = State { grid, robot };

    let instructions: Vec<_> = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("unrecognized character: {c}"),
        })
        .collect();

    for direction in &instructions {
        state.move_robot(*direction);
    }

    let output_1 = state.total_gps();

    let grid = create_grid(&input, true);
    let robot = Position {
        x: robot.x * 2,
        ..robot
    };
    let mut state = State { grid, robot };
    for direction in &instructions {
        state.move_robot(*direction);
    }

    let output_2 = state.total_gps();

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}
