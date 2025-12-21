use std::{collections::HashMap, fs::read_to_string, time::Instant};

struct Circuit<'a>(HashMap<&'a str, Vec<&'a str>>);

impl<'a> Circuit<'a> {
    fn parse(input: &'a str) -> Self {
        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        for line in input.lines() {
            let (from, to) = line.split_once(": ").unwrap();
            for destination in to.split_whitespace() {
                map.entry(from).or_default().push(destination);
            }
        }
        Self(map)
    }

    fn paths_out(&self, start: &str) -> u32 {
        if start == "out" {
            return 1;
        }
        self.0
            .get(start)
            .map(|v| v.iter().map(|s| self.paths_out(s)).sum())
            .unwrap_or_default()
    }

    fn paths_out_restricted(
        &self,
        start: &'a str,
        mut visited_dac: bool,
        mut visited_fft: bool,
        cache: &mut HashMap<(&'a str, bool, bool), u64>,
    ) -> u64 {
        if start == "out" && visited_dac && visited_fft {
            return 1;
        }
        if start == "dac" {
            visited_dac = true;
        }
        if start == "fft" {
            visited_fft = true;
        }
        if let Some(&output) = cache.get(&(start, visited_dac, visited_fft)) {
            return output;
        }
        let output = self
            .0
            .get(start)
            .map(|v| {
                v.iter()
                    .map(|s| self.paths_out_restricted(s, visited_dac, visited_fft, cache))
                    .sum()
            })
            .unwrap_or_default();
        cache.insert((start, visited_dac, visited_fft), output);
        output
    }
}

fn solve(input: &str) -> (u32, u64) {
    let circuit = Circuit::parse(input);
    let output_1 = circuit.paths_out("you");
    let output_2 = circuit.paths_out_restricted("svr", false, false, &mut HashMap::new());
    (output_1, output_2)
}

fn main() {
    let time = Instant::now();
    let input = read_to_string("input").unwrap();
    let (output_1, output_2) = solve(&input);
    println!("part 1: {output_1} part 2: {output_2}");
    println!("time: {}s", time.elapsed().as_secs_f32())
}
