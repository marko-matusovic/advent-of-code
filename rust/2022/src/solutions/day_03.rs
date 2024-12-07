use std::collections::HashSet;

pub fn day() -> u8 {
    3
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());
    Input { lines: lines }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let total: u32 = input
        .lines
        .clone()
        .iter()
        .map(|l| {
            let mid = l.len() / 2;
            let (first, last) = l.split_at(mid);
            let set: HashSet<char> = first.chars().collect();
            let mch: char = last
                .chars()
                .into_iter()
                .find(|ch| set.contains(&ch))
                .unwrap();

            return score(mch);
        })
        .sum();

    println!("Answer is {}", total);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let mut it = input.lines.iter();
    let mut total = 0;
    loop {
        if let Some(first) = it.next() {
            let mid = it.next().unwrap();
            let last = it.next().unwrap();

            let mch: char = first
                .chars()
                .find(|&ch| mid.contains(ch) && last.contains(ch))
                .unwrap();
            total += score(mch);
        } else {
            break;
        }
    }

    println!("Answer is {}", total);
}

fn score(char: char) -> u32 {
    if 'a' <= char && char <= 'z' {
        return char as u32 - 'a' as u32 + 1;
    } else if 'A' <= char && char <= 'Z' {
        return char as u32 - 'A' as u32 + 27;
    }
    println!("Unexpected char {}", char);
    0
}
