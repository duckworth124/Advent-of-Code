use itertools::Itertools;
use num::Rational32;
use std::{fmt::Display, fs::read_to_string, ops::Add, time::Instant};

struct EquationSystem {
    matrix: Vec<Vec<Rational32>>,
    rhs: Vec<Rational32>,
}

impl EquationSystem {
    fn scale_row(&mut self, i: usize, scale_factor: Rational32) {
        self.matrix[i].iter_mut().for_each(|x| *x *= scale_factor);
        self.rhs[i] *= scale_factor;
    }

    fn swap_rows(&mut self, i: usize, j: usize) {
        self.matrix.swap(i, j);
        self.rhs.swap(i, j);
    }

    fn swap_columns(&mut self, i: usize, j: usize) {
        for k in 0..self.matrix.len() {
            self.matrix[k].swap(i, j);
        }
    }

    fn sub_row_from_row(&mut self, sub: usize, sub_from: usize) {
        for k in 0..self.matrix[0].len() {
            let to_sub = self.matrix[sub][k];
            self.matrix[sub_from][k] -= to_sub;
        }
        let to_sub = self.rhs[sub];
        self.rhs[sub_from] -= to_sub
    }

    fn clear_rest_of_column(&mut self, i: usize, j: usize) {
        for k in 0..self.matrix.len() {
            if k == i {
                continue;
            }
            let scale_factor = self.matrix[k][j];
            if scale_factor == 0.into() {
                continue;
            }
            self.scale_row(i, scale_factor);
            self.sub_row_from_row(i, k);
            self.scale_row(i, scale_factor.recip());
        }
    }

    fn put_one(&mut self, finished: usize) {
        for i in finished..self.matrix.len() {
            for j in finished..self.matrix[0].len() {
                if self.matrix[i][j] != 0.into() {
                    self.scale_row(i, self.matrix[i][j].recip());
                    self.swap_columns(j, finished);
                    self.swap_rows(i, finished);
                    return;
                }
            }
        }
    }

    fn gauss_reduce(&mut self) {
        let mut finished = 0;
        loop {
            if finished == self.matrix.len() || finished == self.matrix[0].len() {
                break;
            }
            self.put_one(finished);
            if self.matrix[finished][finished] != 1.into() {
                break;
            }
            self.clear_rest_of_column(finished, finished);
            finished += 1;
        }
    }

    fn solution_space(&self) -> Vec<AffineFunction> {
        let number_of_parameters = self.number_of_parameters();
        let mut output = vec![];
        for (row, &rhs) in self.matrix.iter().zip(&self.rhs) {
            let f = AffineFunction {
                constant: rhs,
                coefficients: row[row.len() - number_of_parameters..]
                    .iter()
                    .map(|x| -x)
                    .collect(),
            };
            output.push(f);
        }
        for i in 0..number_of_parameters {
            let constant = 0.into();
            let mut coefficients = vec![0.into(); number_of_parameters];
            coefficients[i] = 1.into();
            let f = AffineFunction {
                constant,
                coefficients,
            };
            output.push(f);
        }
        output
    }

    fn number_of_parameters(&self) -> usize {
        let number_of_columns = self.matrix[0].len();
        number_of_columns
            - self
                .matrix
                .iter()
                .filter(|v| v.iter().any(|x| *x != 0.into()))
                .count()
    }
}

impl Display for EquationSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self
            .matrix
            .iter()
            .flatten()
            .chain(&self.rhs)
            .copied()
            .map(|x| x.to_string().len() + 1)
            .max()
            .unwrap();

        for k in 0..self.matrix.len() {
            for &c in &self.matrix[k] {
                write!(f, "{c:width$}")?
            }
            write!(f, "|")?;
            write!(f, "{:width$}", self.rhs[k])?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct AffineFunction {
    constant: Rational32,
    coefficients: Vec<Rational32>,
}

impl AffineFunction {
    fn eval(&self, inputs: &[i32]) -> Rational32 {
        inputs
            .iter()
            .copied()
            .map_into()
            .zip(&self.coefficients)
            .map(|(x, &y): (Rational32, &Rational32)| x * y)
            .sum::<Rational32>()
            + self.constant
    }
}

impl Add for AffineFunction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let constant = self.constant + rhs.constant;
        let coefficients = self
            .coefficients
            .into_iter()
            .zip(rhs.coefficients)
            .map(|(x, y)| x + y)
            .collect();
        Self {
            constant,
            coefficients,
        }
    }
}

struct Machine {
    lights: u16,
    joltages: Vec<i32>,
    buttons: Vec<u16>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let mut sections = line.split_whitespace();
        let lights = sections
            .next()
            .unwrap()
            .strip_prefix('[')
            .unwrap()
            .strip_suffix(']')
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .enumerate()
            .map(|(i, x)| (x as u16) << i)
            .sum();

        let joltages: Vec<i32> = sections
            .next_back()
            .unwrap()
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        let buttons = sections
            .map(|s| {
                s.strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .map(|x: u8| 1 << x)
                    .sum()
            })
            .collect();

        Self {
            lights,
            joltages,
            buttons,
        }
    }

    fn min_presses_lights(&self) -> usize {
        for n in 0..self.buttons.len() {
            if self
                .buttons
                .iter()
                .copied()
                .permutations(n)
                .map(|v| v.into_iter().fold(0, |acc, x| acc ^ x))
                .any(|x| x == self.lights)
            {
                return n;
            }
        }
        panic!("no solution found")
    }

    fn min_presses_joltage(self) -> i32 {
        let width = self.buttons.len();
        let height = self.joltages.len();
        let mut matrix = vec![vec![0.into(); width]; height];
        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, x) in row.iter_mut().enumerate() {
                if self.buttons[j] & 1 << i > 0 {
                    *x = 1.into()
                }
            }
        }
        let rhs = self.joltages.into_iter().map(|x| x.into()).collect();
        let mut equations = EquationSystem { matrix, rhs };
        equations.gauss_reduce();
        let solutions = equations.solution_space();
        let output = min_sum(solutions);
        assert!(output.is_integer());
        output.to_integer()
    }
}

fn min_sum(functions: Vec<AffineFunction>) -> Rational32 {
    let num_parameters = functions[0].coefficients.len();
    let mut bounds: Vec<(Rational32, Rational32)> = vec![(0.into(), 1000.into()); num_parameters];
    'outer: loop {
        for f in &functions {
            let max_sum = f.constant
                + f.coefficients
                    .iter()
                    .copied()
                    .zip(&bounds)
                    .map(|(c, x)| c * if c >= 0.into() { x.1 } else { x.0 })
                    .sum::<Rational32>();

            for (i, &c) in f.coefficients.iter().enumerate() {
                if c == 0.into() {
                    continue;
                }
                if c > 0.into() {
                    let max_sum_of_others = max_sum - c * bounds[i].1;
                    let new_lower_bound = (-max_sum_of_others / c).ceil();
                    if new_lower_bound > bounds[i].0 {
                        bounds[i].0 = new_lower_bound;
                        continue 'outer;
                    }
                    continue;
                }
                let max_sum_of_others = max_sum - c * bounds[i].0;
                let new_upper_bound = (-max_sum_of_others / c).floor();
                if new_upper_bound < bounds[i].1 {
                    bounds[i].1 = new_upper_bound;
                    continue 'outer;
                }
            }
        }
        break;
    }

    bounds
        .into_iter()
        .map(|(low, high)| (low.to_integer()..=high.to_integer()))
        .multi_cartesian_product()
        .map(|v| functions.iter().map(|f| f.eval(&v)).collect_vec())
        .filter(|v| v.iter().all(|&x| x.is_integer() && x >= 0.into()))
        .map(|v| v.iter().sum())
        .min()
        .unwrap()
}

fn solve(input: &str) -> (usize, i32) {
    let mut output_1 = 0;
    let mut output_2 = 0;
    for line in input.lines() {
        let machine = Machine::parse(line);
        output_1 += machine.min_presses_lights();
        output_2 += machine.min_presses_joltage();
    }
    (output_1, output_2)
}

fn main() {
    let time = Instant::now();
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
