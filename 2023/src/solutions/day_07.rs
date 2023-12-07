use std::collections::HashMap;

use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
struct Hand {
    cards: Vec<char>,
}
impl Hand {
    fn quality(&self) -> usize {
        let mut counts: HashMap<&char, usize> = self
            .cards
            .iter()
            .unique()
            .map(|c| (c, self.cards.iter().filter(|&cc| c == cc).count()))
            .collect();

        let jk = counts.get(&'*').unwrap_or(&0).to_owned();
        counts.remove(&'*');

        // Five of a kind, where all five cards have the same label: AAAAA
        if counts.iter().any(|(_, &count)| count + jk == 5) || jk == 5 {
            return 0;
        }
        // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        if counts.iter().any(|(_, &count)| count + jk == 4) {
            return 1;
        }
        // Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        // Only possible upgrade with a joker is if 1 joker and 2 pairs
        if (counts.iter().any(|(_, &count)| count == 3)
        && counts.iter().any(|(_, &count)| count == 2))
        || (jk == 1 && counts.iter().filter(|(_, &count)| count == 2).count() == 2)
        {
            return 2;
        }
        // Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        if counts.iter().any(|(_, &count)| count + jk == 3) {
            return 3;
        }
        // Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        // Only possible upgrade with a joker is if 1 joker and 1 pair and 1 card
        if (counts.iter().filter(|(_, &count)| count == 2).count() == 2)
            || (jk == 1 && counts.iter().any(|(_, &count)| count == 2))
        {
            return 4;
        }
        // One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        if counts.iter().any(|(_, &count)| count + jk == 2) {
            return 5;
        }
        // High card, where all cards' labels are distinct: 23456
        return 6;
    }

    fn card_quality(card: &char) -> usize {
        const CARDS: [char; 13] = [
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];
        CARDS.iter().position(|c| c == card).unwrap_or(100)
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.quality().cmp(&other.quality()).then(
            self.cards
                .iter()
                .zip(&other.cards)
                .fold(std::cmp::Ordering::Equal, |ord, (slf, oth)| {
                    ord.then(Hand::card_quality(slf).cmp(&Hand::card_quality(oth)))
                }),
        )
    }
}

#[derive(Debug)]
struct Input {
    lines: Vec<String>,
    hands: Vec<(Hand, usize)>,
    hands_jokers: Vec<(Hand, usize)>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();

    let hands: Vec<(Hand, usize)> = lines
        .iter()
        .map(|l| {
            l.split_once(" ")
                .map(|(cards, bid)| {
                    (
                        Hand {
                            cards: cards.chars().collect(),
                        },
                        bid.parse().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect();

    let hands_jokers: Vec<(Hand, usize)> = hands
        .iter()
        .map(|(hand, bid)| {
            (
                Hand {
                    cards: hand
                        .cards
                        .iter()
                        .map(|c| if *c == 'J' { '*' } else { c.to_owned() })
                        .collect(),
                },
                bid.to_owned(),
            )
        })
        .collect();

    Input {
        lines,
        hands,
        hands_jokers,
    }
}

pub struct Day07;
impl Day for Day07 {
    fn day(&self) -> u8 {
        07
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let total: usize = input
            .hands
            .iter()
            .sorted_by(|(h1, _), (h2, _)| h1.cmp(h2))
            .rev()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum();

        println!("Answer is {}", total);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);
        
        let total: usize = input
            .hands_jokers
            .iter()
            .sorted_by(|(h1, _), (h2, _)| h1.cmp(h2))
            .rev()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum();

        println!("Answer is {}", total);
    }
}
