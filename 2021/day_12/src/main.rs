use std::{
    collections::{BTreeMap, HashMap},
    fs::read_to_string,
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Size {
    Big,
    Small,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Cave {
    connections: Vec<String>,
    size: Size,
}

impl Cave {
    fn new(input: &str) -> Self {
        let size = if input.chars().any(|c| c.is_uppercase()) {
            Size::Big
        } else {
            Size::Small
        };

        let connections = vec![];

        Cave { connections, size }
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
    position: String,
    visited_small_caves: BTreeMap<String, u8>,
}
impl State {
    fn new() -> Self {
        let position = "start".to_string();
        let visited_small_caves = BTreeMap::from([("start".to_string(), 1)]);

        Self {
            position,
            visited_small_caves,
        }
    }

    fn can_visit(&self, cave: String, can_visit_small_cave_twice: bool) -> bool {
        if cave.chars().any(|c| c.is_uppercase()) {
            return true;
        }

        if cave == "start" {
            return false;
        }

        if !self.visited_small_caves.contains_key(&cave) {
            return true;
        }

        if !can_visit_small_cave_twice {
            return false;
        }

        *self.visited_small_caves.values().max().unwrap() < 2
    }
}

struct Caves(HashMap<String, Cave>);

impl Caves {
    fn new(input: &str) -> Self {
        let mut caves = HashMap::new();
        for line in input.lines() {
            let left = line.chars().take_while(|c| *c != '-').collect::<String>();
            let right = line
                .chars()
                .skip_while(|c| *c != '-')
                .skip(1)
                .collect::<String>();

            caves
                .entry(left.clone())
                .or_insert(Cave::new(&left))
                .connections
                .push(right.clone());

            caves
                .entry(right.clone())
                .or_insert(Cave::new(&right))
                .connections
                .push(left);
        }

        Caves(caves)
    }

    fn number_of_paths(
        &self,
        initial_state: State,
        cache: &mut HashMap<State, u32>,
        can_visit_small_cave_twice: bool,
    ) -> u32 {
        if initial_state.position == "end" {
            return 1;
        }

        if let Some(n) = cache.get(&initial_state) {
            return *n;
        }

        let output = self.0[&initial_state.position]
            .connections
            .iter()
            .filter(|s| initial_state.can_visit(s.to_string(), can_visit_small_cave_twice))
            .cloned()
            .map(|next_cave| {
                let mut next_state = State {
                    position: next_cave.clone(),
                    visited_small_caves: initial_state.visited_small_caves.clone(),
                };

                if self.0[&next_cave].size == Size::Small {
                    *next_state.visited_small_caves.entry(next_cave).or_insert(0) += 1;
                }

                next_state
            })
            .map(|state| self.number_of_paths(state, cache, can_visit_small_cave_twice))
            .sum();

        cache.insert(initial_state, output);
        output
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let caves = Caves::new(&input);
    let output_1 = caves.number_of_paths(State::new(), &mut HashMap::new(), false);
    let output_2 = caves.number_of_paths(State::new(), &mut HashMap::new(), true);

    println!("part 1: {output_1} part 2: {output_2}")
}
