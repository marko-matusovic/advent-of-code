use std::{
    collections::{HashMap, HashSet},
    iter::{self, once},
};

use itertools::Itertools;
use regex::Regex;

use super::day_trait::Day;

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    directions: String,
    map: HashMap<String, Node>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let directions = lines[0].clone();
    let re = Regex::new(r"^(\w+) = \((\w+), (\w+)\)$").unwrap();
    let map = lines
        .iter()
        .skip(2)
        .map(|l| {
            let caps = re.captures(l).unwrap();
            (
                caps[1].to_string(),
                Node {
                    id: caps[1].to_string(),
                    left: caps[2].to_string(),
                    right: caps[3].to_string(),
                },
            )
        })
        .collect();
    Input {
        lines,
        directions,
        map,
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

        let mut step = 0;
        let mut pos = "AAA";
        while pos != "ZZZ" {
            pos = match input
                .directions
                .chars()
                .nth(step % input.directions.len())
                .unwrap()
            {
                'L' => &input.map[pos].left,
                'R' => &input.map[pos].right,
                _ => panic!(),
            };
            step += 1;
        }

        println!("Answer is {}", step);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let starts: Vec<String> = input
            .map
            .iter()
            .map(|(n, _)| n.to_owned())
            .filter(|n| n.ends_with('A'))
            .collect();

        dbg!(&starts);

        let loops: Vec<usize> = starts
            .iter()
            .map(|start| {
                let mut cur = start.to_owned();
                let mut step: usize = 0;
                while !cur.ends_with('Z') {
                    cur = match input
                        .directions
                        .chars()
                        .nth(step % input.directions.len())
                        .unwrap()
                    {
                        'L' => input.map[&cur].left.to_owned(),
                        'R' => input.map[&cur].right.to_owned(),
                        _ => panic!(),
                    };
                    step += 1;
                }
                return step;
            })
            .collect();

        println!("Answer is {}", lcm(&loops));
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(numbers: &Vec<usize>) -> usize {
    numbers.iter().fold(1, |a, b| a * b / gcd(a, *b))
}