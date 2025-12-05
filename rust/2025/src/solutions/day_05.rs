use super::day_trait::Day;
use itertools::Itertools;
use std::cmp::max;

#[derive(Debug)]
pub struct Input {
    fresh: Vec<(usize, usize)>,
    stock: Vec<usize>,
}

fn parse_input(raw: &str) -> Input {
    let (raw_fresh, raw_stock) = raw.split_once("\n\n").unwrap();
    Input {
        fresh: raw_fresh
            .split("\n")
            .map(|s| {
                s.split_once('-')
                    .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                    .unwrap()
            })
            .collect(),
        stock: raw_stock.split("\n").map(|s| s.parse().unwrap()).collect(),
    }
}

pub struct Day05;
impl Day for Day05 {
    fn day(&self) -> u8 {
        05
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let count = input
            .stock
            .iter()
            .filter(|&id| input.fresh.iter().any(|(from, to)| from <= id && id <= to))
            .count();

        println!("Answer is {}", count);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());

        let ranges: Vec<(usize, usize)> = parse_input(raw)
            .fresh
            .iter()
            .sorted_by_key(|r| r.0)
            .cloned()
            .collect();

        let mut merged = Vec::new();
        let mut current = ranges[0];

        for next in ranges.iter().skip(1) {
            if next.0 <= current.1 {
                current.1 = max(current.1, next.1);
            } else {
                merged.push(current);
                current = *next;
            }
        }
        merged.push(current);

        let count = merged.iter().map(|(from, to)| to - from + 1).sum::<usize>();

        println!("Answer is {}", count);
    }
}
