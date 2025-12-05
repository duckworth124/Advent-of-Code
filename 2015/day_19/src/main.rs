use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    time::Instant,
};

fn replace_prefix(input: &str, prefix: &str, new: &str) -> Option<String> {
    let input = input.strip_prefix(prefix)?;
    Some(new.to_string() + input)
}

fn all_replacements(input: &str, rules: &HashMap<String, Vec<String>>) -> HashSet<String> {
    (0..=input.len())
        .map(|i| input.split_at(i))
        .flat_map(|(l, r)| {
            all_prefix_replacements(r, rules)
                .into_iter()
                .map(move |r| (l, r))
        })
        .map(|(l, r)| l.to_string() + &r)
        .collect()
}

fn all_replacements_pruned(input: &str, rules: &HashMap<String, Vec<String>>) -> Vec<String> {
    for i in 0..input.len() {
        for j in i..input.len() {
            if let Some(output) = all_replacements_if_forced(input, (i, j), rules) {
                return output;
            }
        }
    }

    (0..=input.len())
        .map(|i| input.split_at(i))
        .filter(|(l, _)| !l.contains("Ar"))
        .flat_map(|(l, r)| {
            all_prefix_replacements(r, rules)
                .into_iter()
                .map(move |r| (l, r))
        })
        .map(|(l, r)| l.to_string() + &r)
        .collect()
}

fn all_replacements_if_forced(
    input: &str,
    (i, j): (usize, usize),
    rules: &HashMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    let s = &input[i..=j];
    if !is_forced_replacement(s, rules) {
        return None;
    }
    let replacements = all_replacements(s, rules);
    if !replacements.is_empty() {
        return Some(
            replacements
                .into_iter()
                .map(|s| format!("{}{s}{}", &input[..i], &input[j + 1..]))
                .collect(),
        );
    }
    None
}

fn is_forced_replacement(s: &str, rules: &HashMap<String, Vec<String>>) -> bool {
    if s.starts_with("Rn")
        && s.ends_with("Ar")
        && s.matches("Rn").count() == 1
        && s.matches("Ar").count() == 1
    {
        return true;
    }

    if rules.contains_key(s) && (s.contains("Ca") || s.contains("Ti")) {
        return true;
    }

    if s == "PB" {
        return true;
    }

    false
}

fn all_prefix_replacements(input: &str, rules: &HashMap<String, Vec<String>>) -> Vec<String> {
    rules
        .iter()
        .flat_map(|(prefix, news)| news.iter().map(move |new| (prefix, new)))
        .filter_map(|(prefix, new)| replace_prefix(input, prefix, new))
        .collect()
}

fn main() {
    let time = Instant::now();
    let input = read_to_string("input").unwrap();
    let start = input.lines().last().unwrap();
    let rules = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (l, r) = l.split_once(" => ").unwrap();
            (l, r)
        })
        .fold(
            HashMap::new(),
            |mut map: HashMap<String, Vec<String>>, (l, r)| {
                map.entry(l.to_string()).or_default().push(r.to_string());
                map
            },
        );

    let output_1 = all_replacements(start, &rules).len();
    let reverse_rules: HashMap<String, Vec<String>> = rules
        .into_iter()
        .flat_map(|(k, v)| v.into_iter().map(move |s| (k.clone(), s)))
        .map(|(l, r)| (r, l))
        .fold(HashMap::new(), |mut map, (l, r)| {
            map.entry(l).or_default().push(r);
            map
        });

    let mut output_2 = 0;
    let mut current = HashSet::from([start.to_string()]);
    while !current.contains("e") {
        assert!(!current.is_empty());
        current = current
            .into_iter()
            .flat_map(|s| all_replacements_pruned(&s, &reverse_rules))
            .collect();
        output_2 += 1
    }

    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
