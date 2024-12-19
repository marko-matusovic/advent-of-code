use std::collections::HashMap;

use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    patterns: Vec<String>,
    rugs: Vec<String>,
}

fn parse_input(raw: &str) -> Input {
    let (raw_patterns, raw_rugs) = raw.split_once("\n\n").unwrap();
    let patterns = raw_patterns.split(", ").map(|p| p.to_owned()).collect_vec();
    let rugs = raw_rugs.split("\n").map(|p| p.to_owned()).collect_vec();
    Input { patterns, rugs }
}

pub struct Day19;
impl Day for Day19 {
    fn day(&self) -> u8 {
        19
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let count = input
            .rugs
            .iter()
            .filter(|rug| can_create(&rug, &input.patterns) > 0)
            .count();

        println!("Answer is {}", count);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let count: usize = input
            .rugs
            .iter()
            .map(|rug| can_create(&rug, &input.patterns))
            .sum();

        println!("Answer is {}", count);
    }
}

fn can_create(rug: &String, patterns: &Vec<String>) -> usize {
    let mut cache = HashMap::new();
    can_create_cached(rug, patterns, &mut cache)
}
fn can_create_cached(
    rug: &String,
    patterns: &Vec<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if rug.len() == 0 {
        return 1;
    }
    if let Some(count) = cache.get(rug) {
        return *count;
    }
    let count = patterns
        .iter()
        .map(|pattern| {
            rug.strip_prefix(pattern).map_or(0, |leftover| {
                can_create_cached(&leftover.to_owned(), patterns, cache)
            })
        })
        .sum();
    cache.insert(rug.to_owned(), count);
    count
}
