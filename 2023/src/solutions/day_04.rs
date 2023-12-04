use std::collections::HashMap;

use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug)]
struct Card {
    winning_nums: Vec<usize>,
    my_nums: Vec<usize>,
}

#[derive(Debug)]
struct Input {
    lines: Vec<String>,
    cards: HashMap<usize, Card>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();

    let cards = lines
        .iter()
        .map(|l| {
            let (id, rest) = l[5..].split_once(": ").unwrap();
            let (win, my) = rest.split_once(" | ").unwrap();
            (
                id.trim().parse::<usize>().unwrap().to_owned(),
                Card {
                    winning_nums: win
                        .replace("  ", " ")
                        .trim()
                        .split(" ")
                        .into_iter()
                        .map(|n| n.parse().unwrap())
                        .collect(),
                    my_nums: my
                        .replace("  ", " ")
                        .trim()
                        .split(" ")
                        .into_iter()
                        .map(|n| n.parse().unwrap())
                        .collect(),
                },
            )
        })
        .collect();

    Input { lines, cards }
}

pub struct Day04;
impl Day for Day04 {
    fn day(&self) -> u8 {
        04
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let sum: usize = input
            .cards
            .iter()
            .map(|(_id, c)| {
                let count = c
                    .my_nums
                    .iter()
                    .filter(|mn| c.winning_nums.contains(mn))
                    .count();
                if count == 0 {
                    0
                } else {
                    2usize.pow((count - 1).try_into().unwrap())
                }
            })
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let mut counts: HashMap<usize, usize> =
            input.cards.keys().into_iter().map(|k| (k.to_owned(), 1)).collect();

        for key in input.cards.keys().sorted() {
            let c = input.cards.get(key).unwrap();
            let count = counts.get(key).unwrap().to_owned();
            let matching = c
                    .my_nums
                    .iter()
                    .filter(|mn| c.winning_nums.contains(mn))
                    .count();
            if matching == 0 {
                continue;
            }
            for i in *key+1..*key+matching+1 {
                counts.entry(i).and_modify(|c| {*c += count} );
            }
        }

        println!("Answer is {}", counts.values().sum::<usize>());
    }
}
