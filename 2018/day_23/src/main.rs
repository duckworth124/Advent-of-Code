use itertools::{Itertools, iproduct};
use std::fs::read_to_string;
use winnow::{
    Parser, Result,
    ascii::{dec_int, dec_uint},
    combinator::{delimited, preceded, separated, separated_pair},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct NanoBot {
    position: (i32, i32, i32),
    range: u32,
}

impl NanoBot {
    const fn is_in_range(self, position: (i32, i32, i32)) -> bool {
        let (x, y, z) = self.position;
        let (x2, y2, z2) = position;
        x.abs_diff(x2) + y.abs_diff(y2) + z.abs_diff(z2) <= self.range
    }

    fn parse(input: &mut &str) -> Result<Self> {
        separated_pair(
            delimited(
                "pos=<",
                separated(3, dec_int::<_, i32, _>, ',').map(|v: Vec<i32>| (v[0], v[1], v[2])),
                '>',
            ),
            ", ",
            preceded("r=", dec_uint),
        )
        .map(|(position, range)| Self { position, range })
        .parse_next(input)
    }

    fn into_region(self) -> Region {
        let constants = iproduct!([1, -1], [1, -1], [1, -1])
            .map(|(x_sign, y_sign, z_sign)| {
                (
                    self.position.0 * x_sign,
                    self.position.1 * y_sign,
                    self.position.2 * z_sign,
                )
            })
            .map(|(x, y, z)| x + y + z - self.range as i32)
            .collect_array()
            .unwrap();
        Region { constants }
    }

    const fn furthest_distance_from_origin(self) -> u32 {
        let (x, y, z) = self.position;
        (x.abs() + y.abs() + z.abs()) as u32 + self.range
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Region {
    //stores the rhs for each inequality involving x, y and z.
    //there are 8 equations, since each variable can be +ve or -ve
    //1st = +x+y+z>=, last = -x-y-z>=
    constants: [i32; 8],
}

impl Region {
    fn smallest_distance_to_origin(self) -> u32 {
        self.constants
            .into_iter()
            .filter(|&x| x >= 0)
            .max()
            .unwrap_or_default() as u32
    }

    fn is_empty(self) -> bool {
        let lows: Vec<i32> = self.constants[..4].to_vec();
        let highs: Vec<i32> = self
            .constants
            .into_iter()
            .rev()
            .take(4)
            .map(|x| -x)
            .collect();
        lows.iter().zip(&highs).any(|(low, high)| low > high)
    }

    fn intersection(self, other: Self) -> Self {
        Self {
            constants: self
                .constants
                .into_iter()
                .zip(other.constants)
                .map(|(x, y)| x.max(y))
                .collect_array()
                .unwrap(),
        }
    }

    fn is_intersecting(self, other: Self) -> bool {
        !self.intersection(other).is_empty()
    }
}

fn adapted_bron_kerbosch(
    r: (Vec<Region>, Region),
    mut p: Vec<Region>,
    x: Vec<Region>,
    best_so_far: &mut (usize, u32),
) {
    let len = r.0.len();
    if p.is_empty() {
        if x.is_empty() {
            if len == best_so_far.0 {
                best_so_far.1 = best_so_far.1.min(r.1.smallest_distance_to_origin())
            }
            if len > best_so_far.0 {
                *best_so_far = (len, r.1.smallest_distance_to_origin());
            }
        }
        return;
    }
    if len + p.len() < best_so_far.0 {
        return;
    }
    let n = p.pop().unwrap();
    let mut new_p = p.clone();
    let mut new_r = r.clone();
    let mut new_x = x.clone();
    new_r.0.push(n);
    new_r.1 = new_r.1.intersection(n);
    new_p.retain(|region| region.is_intersecting(new_r.1));
    new_x.retain(|region| region.is_intersecting(new_r.1));
    adapted_bron_kerbosch(new_r, new_p, new_x, best_so_far);

    let new_r = r;
    let new_p = p.clone();
    let mut new_x = x;
    new_x.push(n);
    adapted_bron_kerbosch(new_r, new_p, new_x, best_so_far)
}

fn solve(input: &str) -> (usize, u32) {
    let nanobots: Vec<NanoBot> = input
        .lines()
        .map(|s| NanoBot::parse.parse(s).unwrap())
        .collect();
    let strongest = nanobots.iter().copied().max_by_key(|n| n.range).unwrap();
    let output_1 = nanobots
        .iter()
        .filter(|n| strongest.is_in_range(n.position))
        .count();
    let furthest_distance = nanobots
        .iter()
        .map(|n| n.furthest_distance_from_origin())
        .max()
        .unwrap();
    let initial_region = NanoBot {
        position: (0, 0, 0),
        range: furthest_distance,
    }
    .into_region();
    let regions: Vec<_> = nanobots.into_iter().map(|n| n.into_region()).collect();
    let mut best_so_far = (0, 0);

    adapted_bron_kerbosch(
        (Vec::new(), initial_region),
        regions,
        Vec::new(),
        &mut best_so_far,
    );

    let output_2 = best_so_far.1;

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
