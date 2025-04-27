use std::fs::read_to_string;
use winnow::{ascii::dec_uint, Parser};

#[derive(Clone, Copy, Debug, Default)]
struct Stats {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl Stats {
    fn to_vec(self) -> Vec<Option<u32>> {
        vec![
            self.children,
            self.cats,
            self.samoyeds,
            self.pomeranians,
            self.akitas,
            self.vizslas,
            self.goldfish,
            self.trees,
            self.cars,
            self.perfumes,
        ]
    }

    fn to_vecs(self) -> (Vec<Option<u32>>, Vec<Option<u32>>, Vec<Option<u32>>) {
        (
            vec![
                self.children,
                self.samoyeds,
                self.akitas,
                self.vizslas,
                self.cars,
                self.perfumes,
            ],
            vec![self.cats, self.trees],
            vec![self.pomeranians, self.goldfish],
        )
    }

    fn could_be(self, other: Self) -> bool {
        could_be_equal(self.to_vec(), other.to_vec())
    }

    fn could_be_2(self, other: Self) -> bool {
        let (eq, gt, lt) = self.to_vecs();
        let (eq2, gt2, lt2) = other.to_vecs();
        could_be_equal(eq, eq2)
            && gt
                .into_iter()
                .zip(gt2)
                .filter_map(|(x, y)| x.zip(y))
                .all(|(x, y)| x > y)
            && lt
                .into_iter()
                .zip(lt2)
                .filter_map(|(x, y)| x.zip(y))
                .all(|(x, y)| x < y)
    }

    fn parse(mut input: &str) -> Self {
        let mut output = Self::default();
        ("Sue ", dec_uint::<&str, u32, ()>, ": ")
            .parse_next(&mut input)
            .unwrap();
        for (field, count) in input.split(", ").map(|s| s.split_once(' ').unwrap()) {
            let count = count.parse().unwrap();
            match field {
                "children:" => output.children = Some(count),
                "cats:" => output.cats = Some(count),
                "samoyeds:" => output.samoyeds = Some(count),
                "pomeranians:" => output.pomeranians = Some(count),
                "akitas:" => output.akitas = Some(count),
                "vizslas:" => output.vizslas = Some(count),
                "goldfish:" => output.goldfish = Some(count),
                "trees:" => output.trees = Some(count),
                "cars:" => output.cars = Some(count),
                "perfumes:" => output.perfumes = Some(count),
                _ => panic!("unrecognized field: {field}"),
            }
        }

        output
    }
}

fn could_be_equal(v1: Vec<Option<u32>>, v2: Vec<Option<u32>>) -> bool {
    v1.into_iter()
        .zip(v2)
        .filter_map(|(x, y)| x.zip(y))
        .all(|(x, y)| x == y)
}

const THE_TRUE_SUE: &str = "Sue 0: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1";

fn main() {
    let input = read_to_string("input").unwrap();
    let target = Stats::parse(THE_TRUE_SUE);
    let output_1 = input
        .lines()
        .map(Stats::parse)
        .position(|s| s.could_be(target))
        .unwrap()
        + 1;

    let output_2 = input
        .lines()
        .map(Stats::parse)
        .position(|s| s.could_be_2(target))
        .unwrap()
        + 1;

    println!("part 1: {output_1} part 2: {output_2}")
}
