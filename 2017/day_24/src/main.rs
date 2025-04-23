use std::fs::read_to_string;

fn max_strength(components: Vec<(u32, u32)>, start: u32) -> u32 {
    (0..components.len())
        .map(|i| {
            let component = components[i];
            let (l, r) = component;
            let strength = l + r;
            if start == l {
                let mut components_copy = components.clone();
                components_copy.swap_remove(i);
                return Some(strength + max_strength(components_copy, r));
            } else if start == r {
                let mut components_copy = components.clone();
                components_copy.swap_remove(i);
                return Some(strength + max_strength(components_copy, l));
            } else {
                None
            }
        })
        .flatten()
        .max()
        .unwrap_or_default()
}

fn max_length_and_strength(components: Vec<(u32, u32)>, start: u32) -> (u32, u32) {
    (0..components.len())
        .map(|i| {
            let component = components[i];
            let (l, r) = component;
            let strength = l + r;
            if start == l {
                let mut components_copy = components.clone();
                components_copy.swap_remove(i);
                let (length, remaining_strength) = max_length_and_strength(components_copy, r);
                return Some((length + 1, strength + remaining_strength));
            } else if start == r {
                let mut components_copy = components.clone();
                components_copy.swap_remove(i);
                let (length, remaining_strength) = max_length_and_strength(components_copy, l);
                return Some((length + 1, strength + remaining_strength));
            } else {
                None
            }
        })
        .flatten()
        .max()
        .unwrap_or_default()
}

fn main() {
    let input = read_to_string("input").unwrap();
    let components: Vec<(u32, u32)> = input
        .lines()
        .map(|s| {
            let (l, r) = s.split_once('/').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();

    let output_1 = max_strength(components.clone(), 0);
    let (_, output_2) = max_length_and_strength(components, 0);
    println!("part 1: {output_1} part 2: {output_2}")
}
