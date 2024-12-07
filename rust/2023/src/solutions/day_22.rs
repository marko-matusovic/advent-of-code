use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use crate::libs::{dir_3d::Dir6, Pos3U};

use super::day_trait::Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Block(Pos3U, Pos3U);

impl Block {
    fn intersect(&self, other: &Block) -> bool {
        let (self_min, self_max) = (self.0, self.1);
        let (other_min, other_max) = (other.0, other.1);

        self_min.0 <= other_max.0
            && self_max.0 >= other_min.0
            && self_min.1 <= other_max.1
            && self_max.1 >= other_min.1
            && self_min.2 <= other_max.2
            && self_max.2 >= other_min.2
    }

    fn offset(&self, dir: Dir6) -> Option<Self> {
        if let Ok(next_0) = (self.0 + dir.dir()).try_into() {
            if let Ok(next_1) = (self.1 + dir.dir()).try_into() {
                return Some(Block(next_0, next_1));
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Input {
    lines: Vec<String>,
    blocks: Vec<Block>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let blocks = lines
        .iter()
        .map(|l| {
            l.split_once("~")
                .map(|(a, b)| {
                    Block(
                        a.split(",").map(|n| n.parse().unwrap()).collect(),
                        b.split(",").map(|n| n.parse().unwrap()).collect(),
                    )
                })
                .unwrap()
        })
        .collect();
    Input { lines, blocks }
}

pub struct Day22;
impl Day for Day22 {
    fn day(&self) -> u8 {
        22
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let mut blocks = input.blocks.clone();

        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;
        for block in blocks.iter() {
            max_x = max(max_x, block.1 .0);
            max_y = max(max_y, block.1 .1);
            max_z = max(max_z, block.1 .2);
        }

        blocks.push(Block(Pos3U(0, 0, 0), Pos3U(max_x, max_y, 0)));

        blocks.sort_by_key(|b| b.0 .2);

        // Compress the gaps between blocks
        for b in 1..blocks.len() {
            // no need to move the platform at 0
            while let Some(moved) = blocks[b].offset(Dir6::D) {
                if (0..b).all(|b2| !moved.intersect(&blocks[b2])) {
                    blocks[b] = moved;
                } else {
                    break;
                }
            }
        }

        blocks.sort_by_key(|b| b.0 .2);

        // Build map of supporting blocks
        let mut supports: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut supported_by: HashMap<usize, Vec<usize>> = HashMap::new();
        for b1 in 2..blocks.len() {
            // 0 is bottom platform, 1 is the bottom most block
            if let Some(block1) = blocks[b1].offset(Dir6::D) {
                for b2 in 1..b1 {
                    // excluding the platform
                    if block1.intersect(&blocks[b2]) {
                        supports.entry(b2).or_insert_with(Vec::new).push(b1);
                        supported_by.entry(b1).or_insert_with(Vec::new).push(b2);
                    }
                }
            }
        }

        // Find decayable blocks
        let mut count: usize = 0;
        for b in 1..blocks.len() {
            if let Some(ss) = supports.get(&b) {
                if ss.iter().all(|b2| supported_by[b2].len() > 1) {
                    count += 1;
                }
            } else {
                count += 1;
            }
        }

        println!("Answer is {}", count);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let mut blocks = input.blocks.clone();

        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_z = 0;
        for block in blocks.iter() {
            max_x = max(max_x, block.1 .0);
            max_y = max(max_y, block.1 .1);
            max_z = max(max_z, block.1 .2);
        }

        blocks.push(Block(Pos3U(0, 0, 0), Pos3U(max_x, max_y, 0)));

        blocks.sort_by_key(|b| b.0 .2);

        // Compress the gaps between blocks
        for b in 1..blocks.len() {
            // no need to move the platform at 0
            while let Some(moved) = blocks[b].offset(Dir6::D) {
                if (0..b).all(|b2| !moved.intersect(&blocks[b2])) {
                    blocks[b] = moved;
                } else {
                    break;
                }
            }
        }

        blocks.sort_by_key(|b| b.0 .2);

        // Build map of supporting blocks
        let mut supports: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut supported_by: HashMap<usize, Vec<usize>> = HashMap::new();
        for b1 in 2..blocks.len() {
            // 0 is bottom platform, 1 is the bottom most block
            if let Some(block1) = blocks[b1].offset(Dir6::D) {
                for b2 in 1..b1 {
                    // excluding the platform
                    if block1.intersect(&blocks[b2]) {
                        supports.entry(b2).or_insert_with(Vec::new).push(b1);
                        supported_by.entry(b1).or_insert_with(Vec::new).push(b2);
                    }
                }
            }
        }

        let mut tally: usize = 0;
        for b in 1..blocks.len() {
            // Count how many blocks are falling after removing b
            let mut falling: HashSet<usize> = vec![b].iter().map(|n| n.to_owned()).collect();
            let mut last_count = 0;
            while falling.len() != last_count {
                last_count = falling.len();
                falling.extend(
                    (2..blocks.len())
                        .filter(|b2| match supported_by.get(b2) {
                            Some(sb) => sb.iter().all(|b3| falling.contains(b3)),
                            None => false,
                        })
                        .collect::<Vec<usize>>(),
                );
            }

            tally += falling.len() - 1;
        }

        println!("Answer is {}", tally);
    }
}
