use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string, iter};

struct Graph {
    vertices: Vec<String>,
    edges: Vec<Vec<bool>>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let vertices = input
            .lines()
            .flat_map(|l| l.split('-'))
            .unique()
            .map(|s| s.to_string())
            .collect_vec();

        let indices: HashMap<&String, usize> =
            vertices.iter().enumerate().map(|(i, s)| (s, i)).collect();

        let mut edges = vec![vec![false; vertices.len()]; vertices.len()];
        input
            .lines()
            .map(|l| l.split('-').collect_tuple().unwrap())
            .map(|(l, r)| (indices[&l.to_string()], indices[&r.to_string()]))
            .for_each(|(l, r)| {
                edges[l][r] = true;
                edges[r][l] = true;
            });

        Self { vertices, edges }
    }

    fn count_triangles_with_t(&self) -> usize {
        self.triangles()
            .iter()
            .filter(|v| v.iter().any(|&i| self.vertices[i].starts_with('t')))
            .count()
    }

    fn largest_clique(&self) -> String {
        self.cliques().take_while(|v| !v.is_empty()).last().unwrap()[0]
            .iter()
            .map(|&i| &self.vertices[i])
            .sorted()
            .join(",")
    }

    fn next_cliques(&self, cliques: &[Vec<usize>]) -> Vec<Vec<usize>> {
        cliques
            .iter()
            .flat_map(|c| {
                let last = c.last().copied().unwrap_or_default();
                (last..self.vertices.len())
                    .filter(|&v| c.iter().copied().all(|u| self.edges[u][v]))
                    .map(|v| c.iter().copied().chain([v]).collect_vec())
            })
            .unique()
            .collect_vec()
    }

    fn cliques(&self) -> impl Iterator<Item = Vec<Vec<usize>>> + '_ {
        iter::successors(Some(vec![vec![]]), |c| Some(self.next_cliques(c)))
    }

    fn triangles(&self) -> Vec<Vec<usize>> {
        self.cliques().nth(3).unwrap()
    }
}

pub fn solve(path: &str) -> (usize, String) {
    let input = read_to_string(path).unwrap();
    let graph = Graph::new(&input);
    let output_1 = graph.count_triangles_with_t();
    let output_2 = graph.largest_clique();
    (output_1, output_2)
}
