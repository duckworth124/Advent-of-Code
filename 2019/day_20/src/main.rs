use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    fs::read_to_string,
};

#[derive(Clone, Copy, Debug)]
enum Tile {
    Open,
    Blocked,
    Portal(usize, usize),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Open => '.',
            Self::Blocked => '#',
            Self::Portal(_, _) => 'O',
        };
        write!(f, "{c}")
    }
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
    goal: (usize, usize),
}

impl Grid {
    fn min_steps(&self) -> u32 {
        let width = self.tiles[0].len();
        let height = self.tiles.len();
        let mut frontier = VecDeque::from([(self.start, 0)]);
        let mut visited = HashSet::new();
        while let Some((mut current_pos, current_cost)) = frontier.pop_front() {
            if !visited.insert(current_pos) {
                continue;
            }
            if current_pos == self.goal {
                return current_cost;
            }
            let (x, y) = current_pos;
            let tile = self.tiles[y][x];
            match tile {
                Tile::Open => {}
                Tile::Blocked => continue,
                Tile::Portal(new_x, new_y) => current_pos = (new_x, new_y),
            }
            let (x, y) = current_pos;
            for (new_x, new_y) in [
                (x.saturating_sub(1), y),
                ((x + 1).min(width - 1), y),
                (x, y.saturating_sub(1)),
                (x, (y + 1).min(height - 1)),
            ] {
                frontier.push_back(((new_x, new_y), current_cost + 1));
            }
        }
        panic!("no path found!")
    }

    fn min_steps_recursive(&self) -> u32 {
        let width = self.tiles[0].len();
        let height = self.tiles.len();
        let mut frontier = VecDeque::from([(self.start, 0, 0)]);
        let mut visited = HashSet::new();
        while let Some((mut current_pos, mut current_depth, current_cost)) = frontier.pop_front() {
            if !visited.insert((current_pos, current_depth)) {
                continue;
            }
            if current_pos == self.goal && current_depth == 0 {
                return current_cost;
            }
            let (x, y) = current_pos;
            let tile = self.tiles[y][x];
            match tile {
                Tile::Open => {}
                Tile::Blocked => continue,
                Tile::Portal(new_x, new_y) => {
                    current_pos = (new_x, new_y);
                    if x == 1 || x == width - 2 || y == 1 || y == height - 2 {
                        if current_depth == 0 {
                            continue;
                        }
                        current_depth -= 1
                    } else {
                        current_depth += 1
                    }
                }
            }
            let (x, y) = current_pos;
            for (new_x, new_y) in [
                (x.saturating_sub(1), y),
                ((x + 1).min(width - 1), y),
                (x, y.saturating_sub(1)),
                (x, (y + 1).min(height - 1)),
            ] {
                frontier.push_back(((new_x, new_y), current_depth, current_cost + 1));
            }
        }
        panic!("no path found!")
    }

    fn parse(input: &str) -> Self {
        let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let height = chars.len();
        let width = chars[0].len();
        let mut grid = Self {
            tiles: vec![],
            start: (0, 0),
            goal: (0, 0),
        };
        for line in chars.iter() {
            let mut row = vec![];
            for &c in line {
                let tile = match c {
                    '#' | ' ' | 'A'..='Z' => Tile::Blocked,
                    '.' => Tile::Open,
                    _ => panic!("unrecognized character: {c:?}"),
                };
                row.push(tile);
            }
            grid.tiles.push(row);
        }
        let mut portals = vec![];
        for x in 0..width {
            for y in 0..height {
                if !chars[y][x].is_ascii_uppercase() {
                    continue;
                }
                if chars[y.saturating_sub(1)][x] == '.' {
                    let first = chars[y][x];
                    let second = chars[y + 1][x];

                    portals.push((first, second, x, y, x, y - 1));
                    continue;
                }
                if chars[y][x.saturating_sub(1)] == '.' {
                    let first = chars[y][x];
                    let second = chars[y][x + 1];
                    portals.push((first, second, x, y, x - 1, y));
                    continue;
                }
                if chars[(y + 1).min(height - 1)][x] == '.' {
                    let first = chars[y - 1][x];
                    let second = chars[y][x];
                    portals.push((first, second, x, y, x, y + 1));
                    continue;
                }
                if chars[y][(x + 1).min(width - 1)] == '.' {
                    let first = chars[y][x - 1];
                    let second = chars[y][x];
                    portals.push((first, second, x, y, x + 1, y));
                    continue;
                }
            }
        }
        portals.sort_unstable();

        while let Some((first, second, portal_x, portal_y, entrance_x, entrance_y)) = portals.pop()
        {
            if first == 'A' && second == 'A' {
                grid.start = (entrance_x, entrance_y);
                continue;
            }
            if first == 'Z' && second == 'Z' {
                grid.goal = (entrance_x, entrance_y);
                continue;
            }
            let (_, _, portal_x_2, portal_y_2, entrance_x_2, entrance_y_2) = portals.pop().unwrap();
            grid.tiles[portal_y][portal_x] = Tile::Portal(entrance_x_2, entrance_y_2);
            grid.tiles[portal_y_2][portal_x_2] = Tile::Portal(entrance_x, entrance_y);
        }
        grid
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}

fn solve(input: &str) -> (u32, u32) {
    let grid = Grid::parse(input);
    (grid.min_steps(), grid.min_steps_recursive())
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
//<666
