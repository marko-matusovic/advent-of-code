use itertools::Itertools;
use regex::Regex;

use crate::libs::Pos3I;

use super::day_trait::Day;

#[derive(Debug)]
struct Stone {
    pos: Pos3I,
    dir: Pos3I,
}

impl Stone {
    fn intersects_2d(&self, other: &Self) -> Option<(f64, f64)> {
        let p0 = self.pos.0 as f64;
        let p1 = self.pos.1 as f64;
        let d0 = self.dir.0 as f64;
        let d1 = self.dir.1 as f64;
        let q0 = other.pos.0 as f64;
        let q1 = other.pos.1 as f64;
        let t0 = other.dir.0 as f64;
        let t1 = other.dir.1 as f64;

        let m1 = d1 / d0;
        let m2 = t1 / t0;
        let b1 = p1 - m1 * p0;
        let b2 = q1 - m2 * q0;
    
        if m1 == m2 {
            // The lines are parallel and will never intersect
            return None;
        }

        let x = (b2 - b1) / (m1 - m2);
        let y = m1 * x + b1;
        
        // Check if the intersection point is in the "future" direction of each line
        if (x - p0) * d0 < 0.0 || (x - q0) * t0 < 0.0 {
            return None;
        }

        Some((x, y))
    }
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    stones: Vec<Stone>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let stones = lines
        .iter()
        .map(|l| {
            let re =
                Regex::new(r"(-?\d+), +(-?\d+), +(-?\d+) +@ +(-?\d+), +(-?\d+), +(-?\d+)").unwrap();
            let caps = re
                .captures(l)
                .unwrap()
                .iter()
                .skip(1)
                .take(6)
                .map(|n| n.unwrap().as_str().parse::<isize>().unwrap())
                .collect_vec();
            Stone {
                pos: Pos3I(caps[0], caps[1], caps[2]),
                dir: Pos3I(caps[3], caps[4], caps[5]),
            }
        })
        .collect_vec();
    Input { lines, stones }
}

pub struct Day24;
impl Day for Day24 {
    fn day(&self) -> u8 {
        24
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let bounds_ok = |n: f64| 200000000000000.0 <= n && n <= 400000000000000.0;
        // let bounds_ok = |n: f64| 7.0 <= n && n <= 27.0; // for ex input

        let mut count = 0;
        for i in 0..input.stones.len() {
            for j in i + 1..input.stones.len() {
                if let Some((u, v)) = input.stones[i].intersects_2d(&input.stones[j]) {
                    if bounds_ok(u) && bounds_ok(v) {
                        count += 1;
                    }
                }
            }
        }

        println!("Answer is {}", count);
    }

    fn part_2(&self, _raw: &str) {
        println!("Day {} part 2", self.day());
        println!("Implemented in Python, because I'm using a solver anyway. ¯\\_(ツ)_/¯");
        println!("Run day_24.py file.");
    }
}
