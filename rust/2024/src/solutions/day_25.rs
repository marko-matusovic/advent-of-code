use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    keys: Vec<u32>,
    locks: Vec<u32>,
}

fn parse_input(raw: &str) -> Input {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for block in raw.split("\n\n") {
        let mut encoded = 0;
        let lines = block
            .split("\n")
            .map(|s| s.chars().collect_vec())
            .skip(1)
            .take(5)
            .collect_vec();

        for x in 0..5 {
            for y in 0..5 {
                if lines[y][x] == '#' {
                    encoded |= 1;
                }
                encoded <<= 1;
            }
        }

        match block.chars().nth(0) {
            Some('#') => {
                keys.push(encoded);
            }
            Some('.') => {
                locks.push(encoded);
            }
            _ => panic!("invalid input"),
        }
    }

    Input { keys, locks }
}

pub struct Day25;
impl Day for Day25 {
    fn day(&self) -> u8 {
        25
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let Input { keys, locks } = parse_input(raw);

        let mut count = 0;
        for lock in locks.iter() {
            for key in keys.iter() {
                if lock & key == 0 {
                    count += 1;
                }
            }
        }

        println!("Answer is {}", count);
    }

    fn part_2(&self, _raw: &str) {
        println!("Day {} part 2", self.day());
        println!("Answer is FREE");
    }
}
