use std::{collections::HashMap, u128};

use itertools::fold;

use super::day_trait::Day;

fn parse_input(raw: &str) -> HashMap<u128, usize> {
    raw.split(" ").map(|d| (d.parse().unwrap(), 1)).collect()
}

pub struct Day11;
impl Day for Day11 {
    fn day(&self) -> u8 {
        11
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());

        let stones = fold(0..25, parse_input(raw), |stones, _| blink(&stones));

        println!("Answer is {}", stones.values().sum::<usize>());
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());

        let stones = fold(0..75, parse_input(raw), |stones, _| blink(&stones));

        println!("Answer is {}", stones.values().sum::<usize>());
    }
}

fn blink(stones: &HashMap<u128, usize>) -> HashMap<u128, usize> {
    let mut next_gen = HashMap::new();
    for (&stone, &count) in stones {
        let sstr = stone.to_string();
        if stone == 0 {
            up_the_count(&mut next_gen, 1, count);
        } else if sstr.len() % 2 == 0 {
            let (l, r) = sstr.split_at(sstr.len() / 2);
            up_the_count(&mut next_gen, l.parse().unwrap(), count);
            up_the_count(&mut next_gen, r.parse().unwrap(), count);
        } else {
            up_the_count(&mut next_gen, stone * 2024, count);
        }
    }
    next_gen
}

fn up_the_count(stones: &mut HashMap<u128, usize>, stone: u128, count: usize) {
    stones
        .entry(stone)
        .and_modify(|c| {
            *c += count;
        })
        .or_insert(count);
}
