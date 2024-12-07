use std::collections::VecDeque;

use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug)]
pub struct Equation {
    test: isize,
    nums: VecDeque<isize>,
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    equations: Vec<Equation>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let equations = lines
        .iter()
        .map(|l| {
            let (test, rest) = l.split_once(": ").unwrap();
            Equation {
                test: test.parse().unwrap(),
                nums: rest.split(" ").map(|n| n.parse().unwrap()).collect(),
            }
        })
        .collect_vec();
    Input { lines, equations }
}

pub struct Day07;
impl Day for Day07 {
    fn day(&self) -> u8 {
        07
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let sum: isize = input
            .equations
            .iter()
            .filter(|eq| valid_eq_p1(eq.test, 0, &eq.nums))
            .map(|eq| eq.test)
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let sum: isize = input
            .equations
            .iter()
            .filter(|eq| valid_eq_p2(eq.test, 0, &eq.nums))
            .map(|eq| eq.test)
            .sum();

        println!("Answer is {}", sum);
    }
}

fn valid_eq_p1(test: isize, accu: isize, nums: &VecDeque<isize>) -> bool {
    if nums.is_empty() {
        return test == accu;
    }

    let mut nums = nums.clone();
    let next = nums.pop_front().unwrap();

    valid_eq_p1(test, accu + next, &nums) || valid_eq_p1(test, accu * next, &nums)
}

fn valid_eq_p2(test: isize, accu: isize, nums: &VecDeque<isize>) -> bool {
    if nums.is_empty() {
        return test == accu;
    }

    let mut nums = nums.clone();
    let next = nums.pop_front().unwrap();

    valid_eq_p2(test, accu + next, &nums)
        || valid_eq_p2(test, accu * next, &nums)
        || valid_eq_p2(test, format!("{}{}", accu, next).parse().unwrap(), &nums)
}

// 3336967112546 low
