use std::collections::HashMap;

use super::day_trait::Day;

#[derive(Debug)]
struct Input {
    lines: Vec<String>,
    bits: Vec<String>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();

    let bits = lines[0].split(",").map(|b| b.to_owned()).collect();

    Input { lines, bits }
}

fn my_hash(seq: &str) -> usize {
    seq.chars()
        .fold(0, |acc, cur| ((acc + cur as usize) * 17) % 256)
}

pub struct Day15;
impl Day for Day15 {
    fn day(&self) -> u8 {
        15
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let sum: usize = input.bits.iter().map(String::as_str).map(my_hash).sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let mut boxes: HashMap<usize, Vec<(String, usize)>> = HashMap::new();
        for instruction in input.bits {
            if let Some((label, focal)) = instruction.split_once("=") {
                let boxx = boxes.entry(my_hash(label)).or_insert_with(Vec::new);
                let focal: usize = focal.parse().unwrap();
                let label = label.to_owned();

                if let Some(pos) = boxx.iter_mut().position(|b| b.0 == label) {
                    boxx.remove(pos);
                    boxx.insert(pos, (label, focal));
                } else {
                    boxx.push((label, focal));
                }

            } else if let Some((label, _)) = instruction.split_once("-") {
                let boxx = boxes.entry(my_hash(label)).or_insert_with(Vec::new);
                let label = label.to_owned();
                boxx.retain(|len| len.0 != label);
            } else {
                panic!()
            }

            // dbg!(&boxes);
        }

        let sum = boxes
            .iter()
            .map(|(bid, boxx)| {
                boxx.iter()
                    .enumerate()
                    .map(|(lid, l)| (bid + 1) * (lid + 1) * l.1)
                    .sum::<usize>()
            })
            .sum::<usize>();

        println!("Answer is {}", sum);
    }
}
