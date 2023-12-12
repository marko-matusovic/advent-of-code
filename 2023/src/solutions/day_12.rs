use std::iter::repeat;

use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug)]
struct HotSpring {
    pattern: String,
    counts: Vec<usize>,
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    hot_springs: Vec<HotSpring>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();

    let hot_springs = lines
        .iter()
        .map(|l| match l.split_once(" ") {
            Some((a, b)) => HotSpring {
                pattern: a.to_owned(),
                counts: b.split(",").map(|n| n.parse().unwrap()).collect(),
            },
            _ => panic!(),
        })
        .collect();

    Input { lines, hot_springs }
}

fn generate_permutations_helper(
    working: usize,
    broken: usize,
    current: Vec<char>,
    permutations: &mut Vec<Vec<char>>,
) {
    if working == 0 && broken == 0 {
        permutations.push(current);
    } else {
        if working > 0 {
            let mut next = current.clone();
            next.push('.');
            generate_permutations_helper(working - 1, broken, next, permutations);
        }
        if broken > 0 {
            let mut next = current.clone();
            next.push('#');
            generate_permutations_helper(working, broken - 1, next, permutations);
        }
    }
}

fn generate_permutations(working: usize, broken: usize) -> Vec<Vec<char>> {
    let mut permutations = Vec::new();
    generate_permutations_helper(working, broken, Vec::new(), &mut permutations);
    permutations
}

fn fill_pattern(pattern: &str, p: &Vec<char>) -> String {
    let mut pter = p.iter();
    pattern
        .chars()
        .map(|ch| match ch {
            '?' => pter.next().unwrap().to_owned(),
            _ => ch,
        })
        .collect::<String>()
}

fn satisfy_pattern(counts: &Vec<usize>, filled_pattern: &str) -> bool {
    *counts
        == filled_pattern
            .split(".")
            .filter(|s| !s.is_empty())
            .map(|s| s.len())
            .collect::<Vec<usize>>()
}

fn count_possible(hs: &HotSpring) -> usize {
    let broken: usize =
        hs.counts.iter().sum::<usize>() - hs.pattern.chars().filter(|&c| c == '#').count();
    let wild = hs.pattern.chars().filter(|&c| c == '?').count();

    let count = generate_permutations(wild - broken, broken)
        .iter()
        .map(|permutation| fill_pattern(&hs.pattern, permutation))
        .filter(|filled_pattern| satisfy_pattern(&hs.counts, filled_pattern))
        .count();

    count
}

pub struct Day12;
impl Day for Day12 {
    fn day(&self) -> u8 {
        12
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let sum: usize = input
            .hot_springs
            .iter()
            .map(count_possible)
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let sum: usize = input
            .hot_springs
            .iter()
            .map(|hs| HotSpring {
                pattern: repeat(hs.pattern.clone()).take(5).collect_vec().join("?"),
                counts: repeat(hs.counts.clone()).flatten().collect_vec(),
            })
            .map(|hs| count_possible(&hs))
            .sum();

        println!("Answer is {}", sum);
    }
}
