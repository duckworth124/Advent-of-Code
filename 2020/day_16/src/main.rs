use itertools::Itertools;
use std::{collections::BTreeSet, fs::read_to_string, ops::RangeInclusive};
use winnow::{Parser, Result, ascii::dec_uint, combinator::separated, token::take_until};

struct Ranges([RangeInclusive<u32>; 2]);

impl Ranges {
    fn parse(input: &mut &str) -> Result<Self> {
        take_until(0.., ':').parse_next(input)?;
        ": ".parse_next(input)?;
        let first_low: u32 = dec_uint(input)?;
        '-'.parse_next(input)?;
        let first_high: u32 = dec_uint(input)?;
        " or ".parse_next(input)?;
        let second_low: u32 = dec_uint(input)?;
        '-'.parse_next(input)?;
        let second_high: u32 = dec_uint(input)?;

        Ok(Self([first_low..=first_high, second_low..=second_high]))
    }

    fn contains(&self, value: u32) -> bool {
        self.0.iter().any(|r| r.contains(&value))
    }
}

fn parse_ranges(input: &mut &str) -> Result<Vec<Ranges>> {
    separated(0.., Ranges::parse, '\n').parse_next(input)
}

struct Ticket(Vec<u32>);

impl Ticket {
    fn apply_permutation(&self, permutation: &Permutation) -> Self {
        Self(permutation.0.iter().map(|i| self.0[*i]).collect())
    }

    fn is_valid(&self, ranges: &[Ranges]) -> bool {
        self.0.iter().all(|&x| ranges.iter().any(|r| r.contains(x)))
    }
}

struct Permutation(Vec<usize>);

struct PartialPermutation(Vec<Option<usize>>);
fn get_valid_permutation(tickets: &[Ticket], ranges: &[Ranges]) -> Permutation {
    let num_fields = ranges.len();
    let mut permutation = PartialPermutation(vec![None; num_fields]);
    let mut i = 0;

    let mut valid = vec![vec![true; num_fields]; num_fields];
    for i in 0..num_fields {
        for j in 0..num_fields {
            for ticket in tickets {
                let range = &ranges[i];
                let value = ticket.0[j];
                if !range.contains(value) {
                    valid[i][j] = false;
                    break;
                }
            }
        }
    }

    let mut seen: BTreeSet<BTreeSet<usize>> = BTreeSet::new();

    while i < ranges.len() {
        permutation.0[i] = Some(permutation.0[i].map(|x| x + 1).unwrap_or_default());

        if seen.contains(&permutation.0.iter().copied().flatten().collect()) {
            continue;
        }

        if permutation.0[i] == Some(ranges.len()) {
            permutation.0[i] = None;
            i = i.checked_sub(1).unwrap();
            seen.insert(permutation.0.iter().copied().flatten().collect());
            continue;
        }
        if !valid[i][permutation.0[i].unwrap()] {
            continue;
        }
        if !permutation.0.iter().copied().flatten().all_unique() {
            continue;
        }

        i += 1;
    }

    Permutation(permutation.0.into_iter().map(|x| x.unwrap()).collect())
}

fn solve(input: &str) -> (u32, u64) {
    let fields = input.split_once("\n\n").unwrap().0;
    let ranges = parse_ranges.parse(fields).unwrap();
    let nearby_tickets = input.split_once("nearby tickets:\n").unwrap().1;
    let output_1 = nearby_tickets
        .lines()
        .flat_map(|line| line.split(','))
        .map(|s| s.parse().unwrap())
        .filter(|&x: &u32| ranges.iter().all(|r| !r.contains(x)))
        .sum();

    let nearby_tickets: Vec<Ticket> = nearby_tickets
        .lines()
        .map(|line| Ticket(line.split(',').map(|s| s.parse().unwrap()).collect()))
        .filter(|t| t.is_valid(&ranges))
        .collect();

    let permutation = get_valid_permutation(&nearby_tickets, &ranges);
    let my_ticket = Ticket(
        input
            .split_once("your ticket:\n")
            .unwrap()
            .1
            .split_once('\n')
            .unwrap()
            .0
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect(),
    );

    let my_ticket = my_ticket.apply_permutation(&permutation);
    let output_2: u64 = my_ticket.0[..6].iter().map(|x| *x as u64).product();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
