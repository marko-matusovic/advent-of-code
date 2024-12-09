use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    rules: Vec<(usize, usize)>,
    books: Vec<Vec<usize>>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let (raw_rules, raw_books) = raw.split_once("\n\n").unwrap();
    let rules = raw_rules
        .split("\n")
        .map(|r| {
            r.split_once("|")
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect_vec();
    let books = raw_books
        .split("\n")
        .map(|b| b.split(",").map(|n| n.parse().unwrap()).collect_vec())
        .collect_vec();

    Input {
        lines,
        rules,
        books,
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

        let sum: usize = input
            .books
            .iter()
            .filter(|&book| {
                input.rules.iter().all(|rule| {
                    if let (Some(p1), Some(p2)) = (
                        book.iter().position(|&n| n == rule.0),
                        book.iter().position(|&n| n == rule.1),
                    ) {
                        p1 < p2
                    } else {
                        true
                    }
                })
            })
            .map(|book| book[book.len() / 2])
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let sum: usize = input
            .books
            .iter()
            .filter_map(|book| {
                let mut corrected_book = book.clone();
                let mut corrected = false;
                let mut modified = true;
                while modified {
                    modified = false;
                    for rule in &input.rules {
                        if let (Some(p1), Some(p2)) = (
                            corrected_book.iter().position(|&n| n == rule.0),
                            corrected_book.iter().position(|&n| n == rule.1),
                        ) {
                            if p1 > p2 {
                                corrected = true;
                                modified = true;
                                (corrected_book[p1], corrected_book[p2]) =
                                    (corrected_book[p2], corrected_book[p1])
                            }
                        }
                    }
                }
                match corrected {
                    true => Some(corrected_book[corrected_book.len() / 2]),
                    false => None,
                }
            })
            .sum();

        println!("Answer is {}", sum);
    }
}
