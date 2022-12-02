use std::collections::HashMap;

pub fn day() -> u8 {
    2
}

#[derive(PartialEq, Clone, Debug)]
enum Hand {
    ROCK = 1,
    PAPER = 2,
    SCISORS = 3,
}

#[derive(Debug)]
enum Outcome {
    LOSS = 0,
    TIE = 3,
    WIN = 6,
}

impl Hand {
    fn parse(ch: &char) -> Hand {
        return match ch {
            'A' | 'X' => Hand::ROCK,
            'B' | 'Y' => Hand::PAPER,
            'C' | 'Z' => Hand::SCISORS,
            _ => panic!("Unexpected play!")
        };
    }

    fn outcome_with(&self, other: Hand) -> Outcome {
        if self == &other {
            return Outcome::TIE;
        }
        if other == self.wins_over() {
            return Outcome::WIN;
        }
        return Outcome::LOSS;
    }

    fn wins_over(&self) -> Hand {
        return match self {
            Hand::ROCK => Hand::SCISORS,
            Hand::PAPER => Hand::ROCK,
            Hand::SCISORS => Hand::PAPER
        }
    }

    fn loses_over(&self) -> Hand {
        return match self {
            Hand::ROCK => Hand::PAPER,
            Hand::PAPER => Hand::SCISORS,
            Hand::SCISORS => Hand::ROCK
        }
    }
}

impl Outcome {
    fn parse(ch: &char) -> Outcome {
        return match ch {
            'X' => Outcome::LOSS,
            'Y' => Outcome::TIE,
            'Z' => Outcome::WIN,
            _ => panic!("Unexpected play!")
        };
    }

    fn with_player(&self, you: &Hand) -> Hand {
        return match self {
            Outcome::LOSS => you.wins_over(),
            Outcome::TIE => you.clone(),
            Outcome::WIN => you.loses_over(),
        }
    }
}
    
impl Clone for Outcome {
    fn clone(&self) -> Self {
        match self {
            Self::LOSS => Outcome::LOSS,
            Self::TIE => Outcome::TIE,
            Self::WIN => Outcome::WIN,
        }
    }
}

#[derive(Debug)]
pub struct Input {
    games: Vec<(char, char)>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<&str> = raw.split("\n").collect();
    println!("Day {} parsing {} lines", day(), lines.len());
    Input {
        games: lines
            .into_iter()
            .map(|l| (l.chars().nth(0).unwrap(), l.chars().nth(2).unwrap()))
            .collect(),
    }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let mut cache: HashMap<(char, char), i32> = HashMap::new();

    let score: i32 = input.games.iter().map(|&(a, b)| {
        return cache.entry((a, b)).or_insert({
            let you = Hand::parse(&a);
            let me = Hand::parse(&b);
            (me.outcome_with(you) as i32) + (me as i32)
        }).clone()
    }).sum();

    println!("Answer is {}", score);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let mut cache: HashMap<(char, char), i32> = HashMap::new();

    let score: i32 = input.games.iter().map(|&(a, b)| {
        return cache.entry((a, b)).or_insert({
            let you = Hand::parse(&a);
            let outcome = Outcome::parse(&b);
            let me = outcome.with_player(&you);
            (outcome as i32) + (me as i32)
        }).clone()
        
    }).sum();

    println!("Answer is {}", score);
}