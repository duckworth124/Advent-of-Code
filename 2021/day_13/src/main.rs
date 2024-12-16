use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
struct Instruction {
    fold_line: Direction,
    coordinate: usize,
}

impl Instruction {
    fn new(input: &str) -> Self {
        let fold_line = if input.chars().any(|c| c == 'x') {
            Direction::Vertical
        } else {
            Direction::Horizontal
        };

        let coordinate: usize = input
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();

        Self {
            fold_line,
            coordinate,
        }
    }
}

#[derive(Clone)]
struct Instructions(Vec<Instruction>);

impl Instructions {
    fn new(input: &str) -> Self {
        let input = input
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1);

        let instructions = input.map(Instruction::new).collect();

        Instructions(instructions)
    }
}

struct Grid(Vec<Vec<bool>>);

impl Grid {
    fn new(input: &str) -> Self {
        let input = input.lines().take_while(|line| !line.is_empty());
        let positions: Vec<(usize, usize)> = input
            .map(|line| {
                let start: String = line.chars().take_while(|c| *c != ',').collect();
                let end: String = line
                    .chars()
                    .skip_while(|c| *c != ',')
                    .skip(1)
                    .collect();

                (start.parse().unwrap(), end.parse().unwrap())
            })
            .collect();

        let max_x = positions.iter().map(|(x, _)| x).max().unwrap();
        let max_y = positions.iter().map(|(_, y)| y).max().unwrap();

        let mut grid = vec![vec![false; max_x + 1]; max_y + 1];
        for (x, y) in positions {
            grid[y][x] = true;
        }

        Grid(grid)
    }

    fn render(&self) {
        for row in &self.0 {
            for b in row {
                if *b {
                    print!("#")
                } else {
                    print!(".")
                }
            }

            println!()
        }
    }

    fn count_squares(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|b| **b).count())
            .sum()
    }

    fn apply(&mut self, instruction: Instruction) {
        let width = self.0[0].len();
        let height = self.0.len();

        match instruction.fold_line {
            Direction::Horizontal => {
                for y in (instruction.coordinate + 1)..height {
                    for x in 0..width {
                        if self.0[y][x] {
                            let new_y = instruction.coordinate - (y - instruction.coordinate);
                            self.0[new_y][x] = true;
                        }
                    }
                }

                self.0.truncate(instruction.coordinate)
            }
            Direction::Vertical => {
                for x in (instruction.coordinate + 1)..width {
                    for y in 0..height {
                        if self.0[y][x] {
                            let new_x = instruction.coordinate - (x - instruction.coordinate);
                            self.0[y][new_x] = true;
                        }
                    }
                }

                for row in self.0.iter_mut() {
                    row.truncate(instruction.coordinate)
                }
            }
        }
    }

    fn apply_multiple(&mut self, instructions: Instructions) {
        for instruction in instructions.0 {
            self.apply(instruction)
        }
    }

    fn apply_first(&mut self, instructions: Instructions) {
        self.apply(instructions.0[0])
    }
}

fn main() {
    let input = read_to_string("practice").unwrap();
    let mut grid = Grid::new(&input);
    let instructions = Instructions::new(&input);
    grid.apply_first(instructions.clone());

    let output_1 = grid.count_squares();

    let mut grid = Grid::new(&input);
    grid.apply_multiple(instructions);

    println!("part 1: {output_1}");
    println!("part 2:");
    grid.render()
}
