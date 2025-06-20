use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
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

fn all_prefix_replacements(input: &str, rules: &HashMap<String, Vec<String>>) -> Vec<String> {
    rules
        .iter()
        .flat_map(|(prefix, news)| news.iter().map(move |new| (prefix, new)))
        .filter_map(|(prefix, new)| replace_prefix(input, prefix, new))
        .collect()
}

fn single_replacement(input: &str, rules: &HashMap<String, Vec<String>>) -> Option<String> {
    all_replacements(input, rules).into_iter().next()
}

fn main() {
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

    let mut current = start.to_string();

    let mut output_2 = 0;
    while current != "e" {
        current = single_replacement(&current, &reverse_rules).unwrap();
        output_2 += 1
    }

    println!("part 1: {output_1} part 2: {output_2}");
}
