use itertools::iproduct;
use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};
use winnow::{
    Parser, Result,
    ascii::dec_uint,
    combinator::{repeat, separated},
    token::any,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Tile {
    data: [[bool; 10]; 10],
    id: u64,
}

impl Tile {
    fn flip(&mut self) {
        self.data.reverse();
    }

    fn transpose(&mut self) {
        for i in 0..10 {
            for j in 0..i {
                let t = self.data[i][j];
                self.data[i][j] = self.data[j][i];
                self.data[j][i] = t;
            }
        }
    }

    fn all_transforms(mut self) -> Vec<Self> {
        let mut output = vec![];
        for _ in 0..4 {
            output.push(self);
            self.flip();
            output.push(self);
            self.transpose()
        }
        output
    }

    fn parse(input: &mut &str) -> Result<Self> {
        "Tile ".parse_next(input)?;
        let id: u64 = dec_uint(input)?;
        ':'.parse_next(input)?;
        '\n'.parse_next(input)?;
        let data = separated(
            10,
            repeat(10, any.map(|c| c == '#'))
                .map(|v: Vec<bool>| -> [bool; 10] { v.try_into().unwrap() }),
            '\n',
        )
        .map(|v: Vec<_>| v.try_into().unwrap())
        .parse_next(input)?;

        Ok(Self { data, id })
    }

    fn fits_left_of(&self, rhs: &Self) -> bool {
        self.data.iter().zip(&rhs.data).all(|(x, y)| x[9] == y[0])
    }

    fn fits_above(&self, rhs: &Self) -> bool {
        self.data[9] == rhs.data[0]
    }
}

struct Grid(VecDeque<VecDeque<Option<Tile>>>);

impl Grid {
    fn try_attach(&mut self, new_tile: Tile) -> bool {
        if self.0.iter().flatten().flatten().next().is_none() {
            self.0 = VecDeque::from([VecDeque::from([Some(new_tile)])]);
            return true;
        }

        for new_tile in new_tile.all_transforms() {
            for (y, row) in self.0.iter().enumerate() {
                for (x, t) in row.iter().enumerate() {
                    if let Some(t) = t {
                        if new_tile.fits_left_of(t) {
                            if x > 0 {
                                self.0[y][x - 1] = Some(new_tile);
                            } else {
                                self.0.iter_mut().for_each(|r| r.push_front(None));
                                self.0[y][0] = Some(new_tile);
                            }
                            return true;
                        }

                        if new_tile.fits_above(t) {
                            if y > 0 {
                                self.0[y - 1][x] = Some(new_tile);
                            } else {
                                let width = self.0[0].len();
                                self.0.push_front(VecDeque::from(vec![None; width]));
                                self.0[0][x] = Some(new_tile);
                            }
                            return true;
                        }

                        if t.fits_left_of(&new_tile) {
                            let width = self.0[0].len();
                            if x < width - 1 {
                            } else {
                                self.0.iter_mut().for_each(|r| r.push_back(None));
                            }

                            self.0[y][x + 1] = Some(new_tile);
                            return true;
                        }

                        if t.fits_above(&new_tile) {
                            let height = self.0.len();
                            if y < height - 1 {
                            } else {
                                let width = self.0[0].len();
                                self.0.push_back(VecDeque::from(vec![None; width]));
                            }

                            self.0[y + 1][x] = Some(new_tile);
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    const fn new() -> Self {
        Self(VecDeque::new())
    }

    fn from_tiles(mut tiles: Vec<Tile>) -> Self {
        let mut output = Self::new();
        while !tiles.is_empty() {
            tiles.retain(|t| !output.try_attach(*t));
        }
        output
    }

    fn corner_product(&self) -> u64 {
        let grid = &self.0;
        grid[0][0].unwrap().id
            * grid[0].back().unwrap().unwrap().id
            * grid.back().unwrap()[0].unwrap().id
            * grid.back().unwrap().back().unwrap().unwrap().id
    }
}

struct Image {
    data: Vec<Vec<bool>>,
    not_sea_monster_points: HashSet<(usize, usize)>,
}

impl Image {
    fn flip(&mut self) {
        self.data.reverse();
        let height = self.data.len();
        self.not_sea_monster_points = self
            .not_sea_monster_points
            .iter()
            .copied()
            .map(|(x, y)| (x, height - y - 1))
            .collect();
    }

    fn transpose(&mut self) {
        let width = self.data.len();
        for i in 0..width {
            for j in 0..i {
                let t = self.data[i][j];
                self.data[i][j] = self.data[j][i];
                self.data[j][i] = t;
            }
        }

        self.not_sea_monster_points = self
            .not_sea_monster_points
            .iter()
            .copied()
            .map(|(x, y)| (y, x))
            .collect();
    }

    fn from_grid(grid: Grid) -> Self {
        let data: Vec<Vec<bool>> = grid
            .0
            .into_iter()
            .flat_map(|row| {
                (row.into_iter().map(|x| x.unwrap()).map(|t| t.data))
                    .map(|a| a[1..9].iter().map(|a| a[1..9].to_vec()).collect::<Vec<_>>())
                    .fold(vec![vec![]; 8], |mut acc, x| {
                        acc.iter_mut().zip(x).for_each(|(x, y)| x.extend(y));
                        acc
                    })
            })
            .collect();

        let width = data.len();

        let not_sea_monster_points = iproduct!(0..width, 0..width)
            .filter(|&(x, y)| data[y][x])
            .collect();

        Self {
            data,
            not_sea_monster_points,
        }
    }

    fn get(&self, (x, y): (usize, usize)) -> Option<bool> {
        self.data.get(y)?.get(x).copied()
    }

    fn is_sea_monster(&self, (x, y): (usize, usize)) -> bool {
        for (dx, dy) in SEA_MONSTER_PATTERN {
            if self.get((x + dx, y + dy)) != Some(true) {
                return false;
            }
        }

        true
    }

    fn remove_sea_monster_points(&mut self, (x, y): (usize, usize)) {
        if self.is_sea_monster((x, y)) {
            for (dx, dy) in SEA_MONSTER_PATTERN {
                self.not_sea_monster_points.remove(&(x + dx, y + dy));
            }
        }
    }

    fn remove_all_sea_monster_points(&mut self) {
        let width = self.data.len();
        for x in 0..width {
            for y in 0..width {
                self.remove_sea_monster_points((x, y));
            }
        }
    }

    fn remove_all_sea_monster_points_including_rotations(&mut self) {
        for _ in 0..4 {
            self.remove_all_sea_monster_points();
            self.flip();
            self.remove_all_sea_monster_points();
            self.transpose();
        }
    }
}

const SEA_MONSTER_PATTERN: [(usize, usize); 15] = [
    (0, 1),
    (1, 2),
    (4, 2),
    (5, 1),
    (6, 1),
    (7, 2),
    (10, 2),
    (11, 1),
    (12, 1),
    (13, 2),
    (16, 2),
    (17, 1),
    (18, 0),
    (18, 1),
    (19, 1),
];

fn solve(input: &str) -> (u64, usize) {
    let tiles: Vec<Tile> = input
        .split("\n\n")
        .map(|l| Tile::parse.parse(l).unwrap())
        .collect();

    let grid = Grid::from_tiles(tiles);
    let output_1 = grid.corner_product();
    let mut image = Image::from_grid(grid);
    image.transpose();

    image.remove_all_sea_monster_points_including_rotations();
    let output_2 = image.not_sea_monster_points.len();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(input.trim());
    println!("part 1: {output_1} part 2: {output_2}")
}

//2484 too high
