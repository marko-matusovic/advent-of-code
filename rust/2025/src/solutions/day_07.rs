use std::collections::HashSet;

use cached::proc_macro::cached;

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    start: usize,
    splitters: Vec<Vec<usize>>,
}

fn parse_input(raw: &str) -> Input {
    Input {
        start: raw.split_once("\n").unwrap().0.find("S").unwrap(),
        splitters: raw
            .lines()
            .skip(1)
            .map(|line| {
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch == '^')
                    .map(|(i, _)| i)
                    .collect()
            })
            .collect(),
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

        let mut beams = HashSet::new();
        beams.insert(input.start);
        let mut total_splits = 0;

        for splitters in input.splitters {
            let mut new_beams = HashSet::new();
            for beam in beams {
                if splitters.contains(&beam) {
                    total_splits += 1;
                    new_beams.insert(beam - 1);
                    new_beams.insert(beam + 1);
                } else {
                    new_beams.insert(beam);
                }
            }
            beams = new_beams;
        }

        println!("Answer is {}", total_splits);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let count = count_timelines(input.start, input.splitters.clone());

        println!("Answer is {}", count);
    }
}

#[cached]
fn count_timelines(beam: usize, splitters: Vec<Vec<usize>>) -> usize {
    if let Some(current_splitters) = splitters.first() {
        let remaining_splitters = splitters[1..].to_vec();
        if current_splitters.contains(&beam) {
            count_timelines(beam - 1, remaining_splitters.clone())
                + count_timelines(beam + 1, remaining_splitters.clone())
        } else {
            count_timelines(beam, remaining_splitters.clone())
        }
    } else {
        1
    }
}
