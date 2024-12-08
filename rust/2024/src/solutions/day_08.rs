use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

use itertools::Itertools;

use crate::libs::Pos2I;

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    dim: Pos2I,
    antennas: HashMap<char, HashSet<Pos2I>>,
    all_antennas: HashSet<Pos2I>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let dim = Pos2I(lines[0].len() as isize, lines.len() as isize);
    let mut antennas = HashMap::new();
    let mut all_antennas = HashSet::new();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            match lines[y].chars().nth(x) {
                Some('.') => {}
                Some(ch) => {
                    let ant = Pos2I(x as isize, y as isize);
                    antennas.entry(ch).or_insert(HashSet::new()).insert(ant);
                    all_antennas.insert(ant);
                }
                _ => panic!(),
            }
        }
    }
    Input {
        lines,
        dim,
        antennas,
        all_antennas,
    }
}

pub struct Day08;
impl Day for Day08 {
    fn day(&self) -> u8 {
        08
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let mut nodes = HashSet::new();

        for ants in input.antennas.values() {
            for combo in ants.iter().combinations(2) {
                let n1 = combo[0].scale(2).sub(combo[1].to_owned());
                if 0 <= n1.0 && n1.0 < input.dim.0 && 0 <= n1.1 && n1.1 < input.dim.1 {
                    nodes.insert(n1);
                }
                let n2 = combo[1].scale(2).sub(combo[0].to_owned());
                if 0 <= n2.0 && n2.0 < input.dim.0 && 0 <= n2.1 && n2.1 < input.dim.1 {
                    nodes.insert(n2);
                }
            }
        }

        println!("Answer is {}", nodes.len());
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let mut nodes = HashSet::new();
        let max_dim = max(input.dim.0, input.dim.1);

        for ants in input.antennas.values() {
            for combo in ants.iter().combinations(2) {
                for i in (-max_dim)..max_dim {
                    let dir = combo[0].sub(combo[1].to_owned());
                    let n1 = combo[0].add(dir.scale(i).to_owned());
                    if 0 <= n1.0 && n1.0 < input.dim.0 && 0 <= n1.1 && n1.1 < input.dim.1 {
                        nodes.insert(n1);
                    }
                }
            }
        }

        println!("Answer is {}", nodes.len());
    }
}
