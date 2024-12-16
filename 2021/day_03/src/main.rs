use std::fs::read_to_string;

struct BinaryNumber {
    digits: Vec<bool>,
}

impl BinaryNumber {
    fn invert(&self) -> Self {
        let digits = self.digits.iter().map(|b| !b).collect();
        Self { digits }
    }

    fn value(&self) -> u32 {
        let mut total = 0;
        for digit in self.digits.iter() {
            total *= 2;
            if *digit {
                total += 1
            }
        }
        total
    }
}
struct Grid(Vec<Vec<bool>>);

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|c| c == '1').collect())
            .collect();
        Grid(grid)
    }

    fn get_column(&self, column: usize) -> impl Iterator<Item = bool> + '_ {
        self.0.iter().map(move |v| v[column])
    }

    fn get_most_common_bit(&self, column: usize) -> bool {
        let height = self.0.len();
        self.get_column(column).filter(|b| *b).count() > height / 2
    }

    fn gamma_rate(&self) -> BinaryNumber {
        let width = self.0[0].len();
        let digits = (0..width)
            .map(|c| self.get_most_common_bit(c))
            .collect();

        BinaryNumber { digits }
    }

    fn epsilon_rate(&self) -> BinaryNumber {
        self.gamma_rate().invert()
    }
}

#[derive(Clone)]
struct Candidates(Vec<Vec<bool>>);

impl Candidates {
    fn new(grid: Grid) -> Self {
        Self(grid.0)
    }

    fn oxygen_rating(self) -> BinaryNumber {
        self.rating(true)
    }

    fn carbon_rating(self) -> BinaryNumber {
        self.rating(false)
    }

    fn rating(mut self, keep_most_common: bool) -> BinaryNumber {
        let len = self.0[0].len();
        for column in 0..len {
            let bits: Vec<_> = self.0.iter().map(|v| v[column]).collect();
            let ones = bits.iter().filter(|b| **b).count();
            let zeroes = bits.len() - ones;

            let bit = (ones < zeroes) ^ keep_most_common;

            self.0.retain(|v| v[column] == bit);

            if self.0.len() == 1 {
                return BinaryNumber {
                    digits: self.0[0].to_vec(),
                };
            }
        }

        panic!("no valid number found")
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let grid = Grid::new(&input);

    let gamma_rate = grid.gamma_rate().value();
    let epsilon_rate = grid.epsilon_rate().value();
    let output_1 = gamma_rate * epsilon_rate;

    let candidates = Candidates::new(grid);
    let oxygen_rating = candidates.clone().oxygen_rating().value();
    let carbon_rating = candidates.carbon_rating().value();
    let output_2 = oxygen_rating * carbon_rating;
    println!("part 1: {output_1 } part 2: {output_2}")
}
