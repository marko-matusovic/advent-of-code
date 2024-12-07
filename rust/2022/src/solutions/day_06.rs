use itertools::Itertools;
use std::collections::LinkedList;

pub fn day() -> u8 {
    6
}

#[derive(Debug)]
pub struct Input {
    chars: Vec<char>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());
    Input {
        chars: lines[0].chars().collect(),
    }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let mut it = input.chars.iter();

    let mut i: usize = 0;
    let mut last: LinkedList<char> = LinkedList::new();
    let flag = loop {
        i += 1;
        let &ch = it.next().unwrap();

        if last.len() == 4 {
            last.pop_front();
        }
        last.push_back(ch);
        if last.iter().unique().count() == 4 {
            break i;
        }
    };

    println!("Answer is {}", flag);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let mut it = input.chars.iter();

    let mut i: usize = 0;
    let mut last: LinkedList<char> = LinkedList::new();
    let flag = loop {
        i += 1;
        let &ch = it.next().unwrap();

        if last.len() == 14 {
            last.pop_front();
        }
        last.push_back(ch);
        if last.iter().unique().count() == 14 {
            break i;
        }
    };

    println!("Answer is {}", flag);
}
