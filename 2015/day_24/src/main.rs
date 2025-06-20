use std::fs::read_to_string;
use tqdm::Iter;

fn is_possible(weights: &[u64], target_weight: u64) -> bool {
    if target_weight == 0 {
        return true;
    }
    if weights.is_empty() {
        return false;
    }
    if is_possible(&weights[1..], target_weight) {
        return true;
    }
    if weights[0] > target_weight {
        return false;
    }
    is_possible(&weights[1..], target_weight - weights[0])
}

fn all_valid_subsets(weights: &[u64], target_weight: u64) -> Vec<Vec<u64>> {
    if target_weight == 0 {
        return vec![vec![]];
    }
    if weights.is_empty() {
        return vec![];
    }
    let mut output = all_valid_subsets(&weights[1..], target_weight);
    if weights[0] <= target_weight {
        output.extend(
            all_valid_subsets(&weights[1..], target_weight - weights[0])
                .into_iter()
                .map(|mut v| {
                    v.push(weights[0]);
                    v
                }),
        );
    }

    output
}

fn solve(input: &str) -> (u64, u64) {
    let weights: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).rev().collect();
    let total_weight: u64 = weights.iter().sum();
    let target_weight = total_weight / 3;
    let output_1 = all_valid_subsets(&weights, target_weight)
        .into_iter()
        .filter(|v| {
            is_possible(
                &weights
                    .iter()
                    .copied()
                    .filter(|w| !v.contains(w))
                    .collect::<Vec<u64>>(),
                target_weight,
            )
        })
        .tqdm()
        .min_by_key(|v| (v.len(), v.iter().map(|x| *x as u128).product::<u128>()))
        .unwrap()
        .iter()
        .product();

    let output_2 = all_valid_subsets(&weights, total_weight / 2)
        .into_iter()
        .map(|v| {
            (
                v.clone(),
                weights
                    .iter()
                    .copied()
                    .filter(|w| !v.contains(w))
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(v1, v2)| is_possible(v1, total_weight / 4) && is_possible(v2, total_weight / 4))
        .map(|(v, _)| v)
        .tqdm()
        .flat_map(|v| all_valid_subsets(&v, total_weight / 4))
        .min_by_key(|v| (v.len(), v.iter().map(|x| *x as u128).product::<u128>()))
        .unwrap()
        .iter()
        .product();

    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
