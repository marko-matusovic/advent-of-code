use super::day_trait::Day;
use crate::libs::{Pos3U, UnionFind};
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Input {
    boxes: Vec<Pos3U>,
}

fn parse_input(raw: &str) -> Input {
    Input {
        boxes: raw
            .lines()
            .map(|l| l.split(",").map(|d| d.parse().unwrap()).collect())
            .collect(),
    }
}

pub struct Day08;
impl Day for Day08 {
    fn day(&self) -> u8 {
        08
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let boxes = parse_input(raw).boxes;

        let mut heap = BinaryHeap::new();
        for (i, a) in boxes.iter().enumerate() {
            for b in boxes.iter().skip(i + 1) {
                heap.push((Reverse(a.dist_n2_sq(*b)), a.clone(), b.clone()));
            }
        }

        let mut circuits = UnionFind::from(boxes.clone());
        for _ in 0..1000 {
            let (_, a, b) = heap.pop().unwrap();
            circuits.union(&a, &b);
        }

        let score: usize = circuits
            .clusters()
            .iter()
            .map(|cluster| cluster.len())
            .sorted()
            .rev()
            .take(3)
            .product();

        println!("Answer is {}", score);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let boxes = parse_input(raw).boxes;

        let mut heap = BinaryHeap::new();
        for (i, a) in boxes.iter().enumerate() {
            for b in boxes.iter().skip(i + 1) {
                heap.push((Reverse(a.dist_n2_sq(*b)), a.clone(), b.clone()));
            }
        }

        let mut circuits = UnionFind::from(boxes.clone());
        let mut n_circuits = boxes.len();

        loop {
            let (_, a, b) = heap.pop().unwrap();
            if let Some(_) = circuits.union(&a, &b) {
                n_circuits -= 1;
                if n_circuits == 1 {
                    println!("Answer is {}", a.0 * b.0);
                    break;
                }
            }
        }
    }
}
