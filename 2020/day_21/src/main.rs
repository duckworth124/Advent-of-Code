use std::{collections::HashSet, fs::read_to_string};

use itertools::Itertools;
use winnow::{
    Parser, Result,
    ascii::alpha1,
    combinator::{delimited, preceded, repeat, separated, terminated},
};

struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

impl<'a> Food<'a> {
    fn parse(input: &mut &'a str) -> Result<Self> {
        let ingredients = repeat(0.., terminated(alpha1, ' ')).parse_next(input)?;
        let allergens = delimited(
            '(',
            preceded("contains ", separated(0.., alpha1, ", ")),
            ')',
        )
        .parse_next(input)?;

        Ok(Self {
            ingredients,
            allergens,
        })
    }
}

fn solve(input: &str) -> (usize, String) {
    let foods: Vec<Food> = input
        .lines()
        .map(|l| Food::parse.parse(l).unwrap())
        .collect();

    let allergens: Vec<&str> = foods
        .iter()
        .flat_map(|f| &f.allergens)
        .copied()
        .unique()
        .collect();
    let possible_ingredients: Vec<Vec<&str>> = allergens
        .iter()
        .map(|a| {
            foods
                .iter()
                .filter(|f| f.allergens.contains(a))
                .map(|f| &f.ingredients)
                .map(|i| i.iter().copied().collect())
                .reduce(|acc: HashSet<&str>, x: HashSet<&str>| &acc & &x)
                .unwrap()
                .into_iter()
                .collect()
        })
        .collect();

    let mut unassigned_ingredients: HashSet<&str> =
        foods.iter().flat_map(|f| &f.ingredients).copied().collect();
    let mut ingredients_without_allergens = unassigned_ingredients.clone();
    let mut allergen_id = 0;
    let mut allergen_assignments: Vec<Option<usize>> = vec![None; allergens.len()];
    loop {
        if allergen_id == allergens.len() {
            allergen_assignments
                .iter()
                .map(|o| o.unwrap())
                .enumerate()
                .map(|(a, i)| possible_ingredients[a][i])
                .for_each(|i| {
                    ingredients_without_allergens.remove(i);
                });

            break;
        }

        let ingredient_id = allergen_assignments[allergen_id]
            .map(|x| x + 1)
            .unwrap_or_default();

        if ingredient_id == possible_ingredients[allergen_id].len() {
            allergen_assignments[allergen_id] = None;
            allergen_id -= 1;
            let ingredient_id = allergen_assignments[allergen_id].unwrap();
            let ingredient = possible_ingredients[allergen_id][ingredient_id];
            unassigned_ingredients.insert(ingredient);
            continue;
        }
        let ingredient = possible_ingredients[allergen_id][ingredient_id];

        allergen_assignments[allergen_id] = Some(ingredient_id);
        if !unassigned_ingredients.remove(ingredient) {
            continue;
        }

        allergen_id += 1;
    }

    let output_1 = foods
        .iter()
        .flat_map(|f| &f.ingredients)
        .copied()
        .filter(|i| ingredients_without_allergens.contains(i))
        .count();

    let output_2 = allergen_assignments
        .into_iter()
        .map(|x| x.unwrap())
        .enumerate()
        .map(|(a, i)| (allergens[a], possible_ingredients[a][i]))
        .sorted_unstable()
        .map(|x| x.1)
        .join(",");

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}");
}
