use crate::libs::UnionFind;

use super::day_trait::Day;
use itertools::Itertools;
use rand::Rng;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone)]
pub struct Edge(String, String);

impl Eq for Edge {}
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0
    }
}
impl Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        vec![self.0.clone(), self.1.clone()]
            .iter()
            .sorted()
            .collect_vec()
            .hash(state);
    }
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    adjacent: HashMap<String, HashSet<String>>,
    edges: HashSet<Edge>,
    nodes: Vec<String>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let mut nodes = HashSet::new();
    let mut adjacent = HashMap::new();
    let mut edges = HashSet::new();
    for line in lines.iter() {
        if let Some((from, tos)) = line.split_once(": ") {
            let tos: Vec<String> = tos.split(" ").map(|s| s.to_owned()).collect();
            adjacent
                .entry(from.to_owned())
                .or_insert_with(HashSet::new)
                .extend(tos.clone());
            for to in tos {
                edges.insert(Edge(from.to_owned(), to.to_owned()));
                adjacent
                    .entry(to.to_owned())
                    .or_insert_with(HashSet::new)
                    .insert(from.to_owned());
                nodes.insert(to.to_owned());
            }
            nodes.insert(from.to_owned());
        }
    }
    Input {
        lines,
        edges,
        adjacent,
        nodes: nodes.iter().cloned().collect(),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State(String, usize);
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.1.cmp(&other.1).reverse())
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn find_path(adjacent: &HashMap<String, HashSet<String>>, from: &String, to: &String) -> Vec<Edge> {
    let mut prev: HashMap<String, String> = HashMap::new();
    let mut dist_to: HashMap<String, usize> = HashMap::new();

    let mut queue = BinaryHeap::new();
    queue.push(State(from.to_owned(), 0));

    while let Some(State(cur_n, cur_l)) = queue.pop() {
        if cur_n == *to {
            break;
        }
        for adj in adjacent.get(&cur_n).unwrap().iter() {
            let dist_to_adj = dist_to.entry(adj.to_owned()).or_insert(usize::MAX);
            if cur_l + 1 < *dist_to_adj {
                *dist_to_adj = cur_l + 1;
                prev.insert(adj.to_owned(), cur_n.clone());
                queue.push(State(adj.clone(), cur_l + 1));
            }
        }
    }

    let mut path: Vec<Edge> = Vec::new();
    let mut cur = to.clone();
    while cur != *from {
        let prev = prev.get(&cur).unwrap().clone();
        path.insert(0, Edge(prev.clone(), cur.clone()));
        cur = prev;
    }

    path
}

pub struct Day25;
impl Day for Day25 {
    fn day(&self) -> u8 {
        25
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let mut rng = rand::thread_rng();
        let random_nodes = (0..50 * 2)
            .map(|_| {
                let n = rng.gen_range(0..input.nodes.len());
                input.nodes[n].clone()
            })
            .collect_vec();

        let paths: Vec<Edge> = random_nodes
            .chunks(2)
            .map(|chunk| find_path(&input.adjacent, &chunk[0], &chunk[1]))
            .flatten()
            .collect();

        let mut edge_count: HashMap<Edge, usize> = HashMap::new();

        for e in paths {
            *edge_count.entry(e).or_insert_with(|| 0) += 1;
        }

        let most_visited = edge_count
            .iter()
            .sorted_by_key(|(_, c)| **c)
            .rev()
            .take(10)
            .map(|(n, _)| n.clone())
            .collect_vec();

        let uf_base = UnionFind::from(input.nodes);

        for i in 0..most_visited.len() {
            for j in i + 1..most_visited.len() {
                for k in j + 1..most_visited.len() {
                    let mut uf = uf_base.clone();
                    for e in input.edges.clone() {
                        if e == most_visited[i] || e == most_visited[j] || e == most_visited[k] {
                            continue;
                        }
                        uf.union(&e.0, &e.1);
                    }
                    let clusters = uf.clusters();
                    if clusters.len() == 2 {
                        println!("Answer is {}", clusters[0].len() * clusters[1].len());
                        return;
                    }
                }
            }
        }
    }

    fn part_2(&self, _raw: &str) {
        println!("Day {} part 2", self.day());
        println!("is free!");
    }
}
