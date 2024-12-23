use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    computers: HashSet<String>,
    connections: HashMap<String, HashSet<String>>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let mut computers = HashSet::new();
    let mut connections = HashMap::new();
    for line in lines {
        let (a, b) = line.split_once("-").unwrap();
        computers.insert(a.to_owned());
        computers.insert(b.to_owned());
        connections
            .entry(a.to_owned())
            .or_insert(HashSet::new())
            .insert(b.to_owned());
        connections
            .entry(b.to_owned())
            .or_insert(HashSet::new())
            .insert(a.to_owned());
    }
    Input {
        computers,
        connections,
    }
}

pub struct Day23;
impl Day for Day23 {
    fn day(&self) -> u8 {
        23
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let Input {
            computers,
            connections,
        } = parse_input(raw);

        let mut cliques: HashSet<String> = HashSet::new();

        for computer in computers.iter().filter(|c| c.starts_with("t")) {
            for neighbors in connections[computer].iter().combinations(2) {
                if connections[neighbors[0]].contains(neighbors[1]) {
                    cliques.insert(once(computer).chain(neighbors).sorted().join(","));
                }
            }
        }

        println!("Answer is {}", cliques.len());
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let Input {
            computers,
            connections,
        } = parse_input(raw);

        let mut max_found = Vec::new();
        for c in computers.iter().sorted() {
            let found = max_clique(&vec![c.to_owned()], &max_found, &connections);
            if max_found.len() < found.len() {
                max_found = found;
            }
        }

        println!("Answer is {}", max_found.iter().sorted().join(","));
    }
}

fn is_clique(nodes: &Vec<String>, connections: &HashMap<String, HashSet<String>>) -> bool {
    for (i, node) in nodes.iter().enumerate() {
        for other in nodes.iter().skip(i + 1) {
            if !connections[node].contains(other) {
                return false;
            }
        }
    }
    true
}

fn max_clique(
    nodes: &Vec<String>,
    max_found: &Vec<String>,
    connections: &HashMap<String, HashSet<String>>,
) -> Vec<String> {
    if !is_clique(nodes, connections) {
        return max_found.to_owned();
    }

    let common_neighbors = nodes
        .iter()
        .map(|n| connections[n].to_owned())
        .reduce(|a, b| a.intersection(&b).map(|s| s.to_owned()).collect())
        .unwrap_or(HashSet::new());
    let filtered_neighbors = common_neighbors
        .iter()
        .sorted()
        .skip_while(|&n| n < nodes.last().unwrap())
        .collect_vec();
    
    if filtered_neighbors.is_empty() {
        return nodes.to_owned();
    }
    if nodes.len() + filtered_neighbors.len() <= max_found.len() {
        return max_found.to_owned();
    }
    
    let mut max_found = max_found.to_owned();
    for n in filtered_neighbors {
        let found = max_clique(
            &vec![nodes.to_owned(), vec![n.to_owned()]].concat(),
            &max_found,
            connections,
        );
        if found.len() > max_found.len() {
            max_found = found;
        }
    }
    max_found
}
