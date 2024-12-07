use itertools::Itertools;
use regex::Regex;

use super::day_trait::Day;

pub struct Day03;
impl Day for Day03 {
    fn day(&self) -> u8 {
        03
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());

        let re = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();
        let sum: usize = re
            .captures_iter(raw)
            .map(|c| {
                let (_, [a, b]) = c.extract();
                a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
            })
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());

        let re = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();
        let sum: usize = re
            .captures_iter(
                &raw.split("do()")
                    .map(|part| part.split("don't()").collect_vec()[0])
                    .collect_vec()
                    .join(" "),
            )
            .map(|c| {
                let (_, [a, b]) = c.extract();
                a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
            })
            .sum();

        println!("Answer is {}", sum);
    }
}
