use std::iter::zip;

use itertools::Itertools;
use pathfinding::num_traits::abs;

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    left: Vec<isize>,
    right: Vec<isize>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in &lines {
        let (a, b) = line.split_once("   ").unwrap();
        left.push(a.parse().unwrap());
        right.push(b.parse().unwrap());
    }
    Input { lines, left, right }
}

pub struct Day01;
impl Day for Day01 {
    fn day(&self) -> u8 {
        01
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let sum: isize = zip(input.left.iter().sorted(), input.right.iter().sorted())
            .map(|(a, b)| abs(a - b))
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let sum: isize = input
            .left
            .iter()
            .map(|l| input.right.iter().filter(|&r| r == l).count() as isize * l)
            .sum();

        println!("Answer is {}", sum);
    }
}
