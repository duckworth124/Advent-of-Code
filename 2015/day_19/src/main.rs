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

fn min_steps(start: &str, target: &str, rules: &HashMap<String, Vec<String>>) -> u32 {
    let mut frontier: HashSet<String> = HashSet::from_iter(vec![start.to_string()]);
    let mut steps = 0;
    loop {
        if frontier.contains(target) {
            return steps;
        }
        dbg!(steps);
        dbg!(frontier.len());
        steps += 1;
        frontier = frontier
            .into_iter()
            .flat_map(|s| all_replacements(&s, rules))
            .collect();
    }
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
    /*let reverse_rules: HashMap<String, Vec<String>> = rules
    .into_iter()
    .map(|(k, v)| v.into_iter().map(move |s| (k.clone(), s)))
    .flatten()
    .map(|(l, r)| (r, l))
    .fold(HashMap::new(), |mut map, (l, r)| {
        map.entry(l.to_string()).or_insert(vec![]).push(r);
        map
    });*/

    let output_2 = min_steps("e", start, &rules);

    println!("part 1: {output_1} part 2: {output_2}");
}
