use std::fs::read_to_string;

use itertools::iproduct;

struct Grid(Vec<Vec<i32>>);

impl Grid {
    fn new(serial: i32) -> Self {
        let mut grid = vec![vec![0; 300]; 300];
        grid.iter_mut().enumerate().for_each(|(y, p)| {
            p.iter_mut().enumerate().for_each(|(x, p)| {
                let rack_id = (x + 1 + 10) as i32;
                let power = (rack_id * (y as i32 + 1) + serial) * rack_id;
                let power = (power % 1000) / 100 - 5;
                *p = power
            })
        });
        Self(grid)
    }

    fn best_coords(&self) -> (usize, usize) {
        iproduct!(0..297, 0..297)
            .max_by_key(|&(x, y)| {
                self.0[y..y + 3]
                    .iter()
                    .flat_map(|v| &v[x..x + 3])
                    .sum::<i32>()
            })
            .map(|(x, y)| (x + 1, y + 1))
            .unwrap()
    }

    fn best_coords_2(&self) -> (usize, usize, usize) {
        iproduct!(0..297, 0..297)
            .flat_map(|(x, y)| (0..=300 - (x.max(y))).map(move |s| (x, y, s)))
            .max_by_key(|&(x, y, s)| {
                if x + s > 300 {
                    return 0;
                }
                if y + s > 300 {
                    return 0;
                }
                self.0[y..y + s]
                    .iter()
                    .flat_map(|v| &v[x..x + s])
                    .sum::<i32>()
            })
            .map(|(x, y, s)| (x + 1, y + 1, s))
            .unwrap()
    }
}

fn solve(input: &str) -> ((usize, usize), (usize, usize, usize)) {
    let serial: i32 = input.trim().parse().unwrap();
    let grid = Grid::new(serial);
    let output_1 = grid.best_coords();
    let output_2 = grid.best_coords_2();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1:?} part 2: {output_2:?}")
}
