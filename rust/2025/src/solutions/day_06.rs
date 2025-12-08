use std::iter::zip;

use itertools::Itertools;

use crate::libs::Grid2;

use super::day_trait::Day;

pub struct Day06;
impl Day for Day06 {
    fn day(&self) -> u8 {
        06
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());

        let lines = raw
            .lines()
            .map(|line| line.split(" ").into_iter().filter(|s| !s.is_empty()));
        let nums: Vec<Vec<usize>> = lines
            .clone()
            .take(4)
            .map(|line| {
                line.into_iter()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let ops: Vec<char> = lines
            .last()
            .unwrap()
            .into_iter()
            .map(|s| s.chars().nth(0).unwrap())
            .collect();

        let summ: usize = zip(Grid2(nums).transpose().0.iter(), ops.iter())
            .map(|(ns, op)| exec(op, &ns))
            .sum();

        println!("Answer is {}", summ);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());

        let grid = Grid2(
            raw.lines()
                .take(4)
                .into_iter()
                .map(|l| l.chars().into_iter().collect())
                .collect(),
        );
        let lines = grid
            .transpose()
            .0
            .into_iter()
            .rev()
            .map(|line| line.into_iter().filter(|&ch| ch != ' ').collect::<String>())
            .join("\n");

        let nums: Vec<Vec<usize>> = lines
            .split("\n\n")
            .map(|group| group.lines().map(|line| line.parse().unwrap()).collect())
            .collect();

        let ops: Vec<char> = raw
            .lines()
            .last()
            .unwrap()
            .chars()
            .filter(|&ch| ch != ' ')
            .rev()
            .collect();

        let summ: usize = zip(nums.iter(), ops.iter())
            .map(|(ns, op)| exec(op, &ns))
            .sum();

        println!("Answer is {}", summ);
    }
}

fn exec(op: &char, nums: &Vec<usize>) -> usize {
    match op {
        '+' => nums.iter().sum(),
        '*' => nums.iter().product(),
        _ => panic!("unexpected op"),
    }
}
