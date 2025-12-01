use std::{collections::HashMap, fs::read_to_string};

use winnow::{
    Parser, Result,
    ascii::{alpha1, dec_uint, line_ending},
    combinator::{separated, separated_pair, seq},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Material<'a>(&'a str);

impl<'a> Material<'a> {
    fn parse(input: &mut &'a str) -> Result<Self> {
        alpha1.map(Self).parse_next(input)
    }

    fn is_ore(self) -> bool {
        self.0 == "ORE"
    }
}

struct Recipe<'a> {
    product: Material<'a>,
    product_quantity: u64,
    ingredients: Vec<(u64, Material<'a>)>,
}

impl<'a> Recipe<'a> {
    fn parse(input: &mut &'a str) -> Result<Self> {
        seq! {Self {ingredients:
        separated(1..,separated_pair(dec_uint, ' ', Material::parse), ", "),
        _: " => ",
        product_quantity: dec_uint,
        _: ' ',
        product: Material::parse
        }}
        .parse_next(input)
    }
}

struct Blueprint<'a>(HashMap<Material<'a>, Recipe<'a>>);

impl<'a> Blueprint<'a> {
    fn parse(input: &mut &'a str) -> Result<Self> {
        separated(0.., Recipe::parse, line_ending)
            .map(|v: Vec<Recipe>| v.into_iter().map(|r| (r.product, r)).collect())
            .map(Self)
            .parse_next(input)
    }

    fn total_ore_required(&self, fuel_amount: u64) -> u64 {
        let mut requirements: HashMap<Material, i64> =
            HashMap::from([(Material("FUEL"), fuel_amount as i64)]);
        while let Some((&material, &required_amount)) =
            requirements.iter().find(|(m, c)| !m.is_ore() && **c > 0)
        {
            let recipe = &self.0[&material];
            let n = required_amount / recipe.product_quantity as i64
                + ((required_amount % recipe.product_quantity as i64 > 0) as i64);
            for &(count, ingredient) in &recipe.ingredients {
                *requirements.entry(ingredient).or_default() += n * count as i64;
            }
            *requirements.entry(material).or_default() -= n * recipe.product_quantity as i64;
        }
        requirements[&Material("ORE")] as u64
    }
}

fn solve(input: &str) -> (u64, u64) {
    let blueprint = Blueprint::parse.parse(input.trim()).unwrap();
    let output_1 = blueprint.total_ore_required(1);
    let mut low = 0;
    let mut high = 1;
    while blueprint.total_ore_required(high) <= 1_000_000_000_000 {
        high *= 2
    }
    while high > low + 1 {
        let mid = low + (high - low) / 2;
        if blueprint.total_ore_required(mid) <= 1_000_000_000_000 {
            low = mid
        } else {
            high = mid
        }
    }
    let output_2 = low;

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
