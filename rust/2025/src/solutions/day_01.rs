use super::day_trait::Day;
use pathfinding::num_traits::abs;

#[derive(Debug)]
pub struct Input {
    rotations: Vec<i32>,
}

fn parse_input(raw: &str) -> Input {
    let rotations: Vec<i32> = raw
        .split("\n")
        .map(|s| {
            if s.trim().starts_with("L") {
                -s[1..].parse::<i32>().unwrap()
            } else {
                s[1..].parse::<i32>().unwrap()
            }
        })
        .collect();
    Input { rotations }
}

pub struct Day01;
impl Day for Day01 {
    fn day(&self) -> u8 {
        01
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let mut position = 50;
        let mut count = 0;

        for rot in input.rotations {
            position = (position + rot) % 100;
            if position == 0 {
                count += 1;
            }
        }

        println!("Answer is {}", count);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);
        let mut position = 50;
        let mut count = 0;

        for rot in input.rotations {
            let sign = rot / abs(rot);
            for _ in 0..abs(rot) {
                position = (position + sign) % 100;
                if position == 0 {
                    count += 1;
                }
            }
        }

        println!("Answer is {}", count);
    }
}
