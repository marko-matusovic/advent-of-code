use std::{
    collections::{BinaryHeap, HashSet},
    ops::Add,
};

use crate::libs::{dir_2d::Dir4, Pos2U};

use super::day_trait::Day;

fn parse_input(raw: &str) -> Vec<Pos2U> {
    raw.split("\n")
        .map(|s| {
            let (x, y) = s.split_once(",").unwrap();
            Pos2U(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    pos: Pos2U,
    dist: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.dist + (70 + 70 - self.pos.0 - self.pos.1))
            .cmp(&(other.dist + (70 + 70 - other.pos.0 - other.pos.1)))
            .reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day18;
impl Day for Day18 {
    fn day(&self) -> u8 {
        18
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let bytes = parse_input(raw);

        let obstacles: HashSet<Pos2U> = bytes.iter().take(1024).map(|p| p.to_owned()).collect();

        let steps = shortest_path(&obstacles);

        println!("Answer is {}", steps.unwrap());
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let bytes = parse_input(raw);

        for i in 0..bytes.len() {
            let obstacles: HashSet<Pos2U> = bytes.iter().take(i).map(|p| p.to_owned()).collect();
            if shortest_path(&obstacles).is_none() {
                println!("Answer is {},{}", bytes[i-1].0, bytes[i-1].1);
                break;
            }
        }
    }
}

fn shortest_path(obstacles: &HashSet<Pos2U>) -> Option<usize> {
    let dest = Pos2U(70, 70); 
    let mut visited: HashSet<Pos2U> = HashSet::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        pos: Pos2U(0, 0),
        dist: 0,
    });
    while let Some(State { pos, dist }) = queue.pop() {
        if pos == dest {
            return Some(dist);
        }

        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);

        for dx in Dir4::all() {
            if let Ok(Pos2U(x, y)) = pos.add(dx.dir()).try_into() {
                if x > dest.0 || y > dest.1 {
                    continue;
                }
                if !obstacles.contains(&Pos2U(x, y)) {
                    queue.push(State {
                        pos: Pos2U(x, y),
                        dist: dist + 1,
                    });
                }
            }
        }
    }
    None
}
