use std::{cmp::Ordering, collections::LinkedList};

pub fn day() -> u8 {
    13
}

#[derive(Debug)]
pub struct Input {
    pairs: Vec<(Packet, Packet)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
enum Packet {
    INT(i64),
    LIST(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Packet::INT(n) => match other {
                Packet::INT(m) => n.partial_cmp(&m),
                Packet::LIST(brr) => Some(compare_packet_list(&vec![Packet::INT(n.clone())], brr)),
            },
            Packet::LIST(arr) => match other {
                Packet::INT(m) => Some(compare_packet_list(arr, &vec![Packet::INT(m.clone())])),
                Packet::LIST(brr) => Some(compare_packet_list(arr, brr)),
            },
        }
    }
}

fn compare_packet_list(arr: &Vec<Packet>, brr: &Vec<Packet>) -> Ordering {
    let mut arr_iter = arr.iter();
    let mut brr_iter = brr.iter();
    return loop {
        let a = arr_iter.next();
        let b = brr_iter.next();
        if a.is_none() && b.is_none() {
            break Ordering::Equal;
        } else if a.is_none() && b.is_some() {
            break Ordering::Less;
        } else if a.is_some() && b.is_none() {
            break Ordering::Greater;
        }
        let ord = a.unwrap().partial_cmp(b.unwrap()).unwrap();
        if ord != Ordering::Equal {
            break ord;
        }
    };
}

pub fn parse_input(raw: &str) -> Input {
    let pairs_raw: Vec<String> = raw.split("\n\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} pairs", day(), pairs_raw.len());

    let pairs: Vec<(Packet, Packet)> = pairs_raw
        .iter()
        .map(|pair| {
            let (left, right) = pair.split_once("\n").unwrap();

            let mut left_iter: LinkedList<char> = left.chars().collect();
            let mut right_iter: LinkedList<char> = right.chars().collect();

            (parse_packet(&mut left_iter), parse_packet(&mut right_iter))
        })
        .collect();

    Input { pairs }
}

fn parse_packet(reader: &mut LinkedList<char>) -> Packet {
    if *reader.front().unwrap() == '[' {
        let mut content: Vec<Packet> = Vec::new();
        let mut digit: Vec<char> = Vec::new();
        let mut item: Option<Packet> = None;
        reader.pop_front();
        loop {
            // let input: String = reader.iter().collect();
            // println!("parsing: {}", input);
            let &ch = reader.front().unwrap();
            match ch {
                ',' => {
                    if digit.len() > 0 {
                        item = Some(Packet::INT(
                            digit.iter().collect::<String>().parse().unwrap(),
                        ));
                        digit.clear();
                    }
                    content.push(item.clone().unwrap());
                    item = None;
                    reader.pop_front();
                }
                '[' => item = Some(parse_packet(reader)),
                ']' => {
                    if digit.len() > 0 {
                        item = Some(Packet::INT(
                            digit.iter().collect::<String>().parse().unwrap(),
                        ));
                    }
                    match item.clone() {
                        Some(i) => content.push(i),
                        None => {}
                    }
                    reader.pop_front();
                    return Packet::LIST(content);
                }
                _ => {
                    digit.push(ch);
                    reader.pop_front();
                }
            }
        }
    } else {
        panic!(
            "unexpected rest of input {}",
            reader.iter().collect::<String>()
        );
    }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let total: usize = input
        .pairs
        .iter()
        .enumerate()
        .filter(|(_i, (left, right))| left < right)
        .map(|(i, _)| i + 1)
        .sum();

    println!("Answer is {}", total);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let start = Packet::LIST(vec![Packet::LIST(vec![Packet::INT(2)])]);
    let end = Packet::LIST(vec![Packet::LIST(vec![Packet::INT(6)])]);
    let mut lower = 0;
    let mut higher = 0;

    input
        .pairs
        .iter()
        .flat_map(|(left, right)| vec![left, right])
        .for_each(|packet| {
            if packet.clone() < start {
                lower += 1
            } else if end < packet.clone() {
                higher += 1;
            }
        });

    let id_start = lower + 1;
    let id_end = (input.pairs.len() * 2) + 2 - higher;

    println!("Answer is {}", id_start * id_end);
}
