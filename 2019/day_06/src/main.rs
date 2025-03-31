use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

struct OrbitMap {
    objects: HashMap<String, Vec<String>>,
}

impl OrbitMap {
    fn new(data: &str) -> Self {
        let mut objects: HashMap<String, Vec<String>> = HashMap::new();
        for line in data.lines() {
            let (l, r) = line.split_once(')').unwrap();
            objects
                .entry(l.to_string())
                .or_default()
                .push(r.to_string());
        }

        Self { objects }
    }

    fn all_orbits(&self, root: &str) -> usize {
        self.objects
            .get(root)
            .into_iter()
            .flatten()
            .map(|s| self.all_orbits(s))
            .map(|x| x + 1)
            .sum()
    }

    fn checksum(&self) -> usize {
        self.objects.keys().map(|s| self.all_orbits(s)).sum()
    }

    fn adjacents(&self, object: &str) -> impl Iterator<Item = &String> {
        self.objects
            .iter()
            .filter(|(_, v)| v.contains(&object.to_string()))
            .map(|x| x.0)
            .chain(self.objects.get(object).into_iter().flatten())
    }

    fn min_distance(&self) -> usize {
        let start = self
            .objects
            .iter()
            .find(|(_, v)| v.contains(&"YOU".to_string()))
            .unwrap()
            .0;
        let end = self
            .objects
            .iter()
            .find(|(_, v)| v.contains(&"SAN".to_string()))
            .unwrap()
            .0;

        let mut frontier = vec![start];
        let mut visited = HashSet::new();
        for steps in 0.. {
            if frontier.contains(&end) {
                return steps;
            }
            frontier = frontier
                .into_iter()
                .flat_map(|s| self.adjacents(s))
                .filter(|s| visited.insert(*s))
                .collect();
        }

        panic!("no path found")
    }
}

fn solve(input: &str) -> (usize, usize) {
    let orbit_map = OrbitMap::new(input);
    let output_1 = orbit_map.checksum();
    let output_2 = orbit_map.min_distance();
    (output_1, output_2)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}")
}
