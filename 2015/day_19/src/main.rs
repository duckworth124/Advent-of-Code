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

fn all_replacements_pruned(input: &str, rules: &HashMap<String, Vec<String>>) -> Vec<String> {
    let things_we_reduce_immediately = ["Ca", "Ti", "Y"];
    (0..=input.len())
        .map(|i| input.split_at(i))
        .filter(|(l, _)| !l.contains("Ar"))
        .filter(|(l, _)| !l.contains("Al"))
        .filter(|(l, _)| {
            rules
                .keys()
                .filter(|s| things_we_reduce_immediately.iter().any(|x| s.contains(x)))
                .all(|s| !l.contains(s))
        })
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

    let mut output_2 = 0;
    let mut current = HashSet::from([start.to_string()]);
    while !current.contains("e") {
        assert!(!current.is_empty());
        current = current
            .into_iter()
            .flat_map(|s| all_replacements_pruned(&s, &reverse_rules))
            .collect();
        println!(
            "{}",
            current
                .iter()
                .map(|s| s.chars().filter(|&c| c == 'R').count())
                .max()
                .unwrap_or_default()
        );
        output_2 += 1
    }

    println!("part 1: {output_1} part 2: {output_2}");
}
