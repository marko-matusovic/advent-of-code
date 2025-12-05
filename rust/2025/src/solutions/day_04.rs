use super::day_trait::Day;
use crate::libs::dir_2d::Dir8;
use crate::libs::{Grid2, Pos2U};
use std::ops::Add;

#[derive(Debug)]
pub struct Input {
    paper: Grid2<char>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    Input {
        paper: Grid2::from(&lines),
    }
}

pub struct Day04;
impl Day for Day04 {
    fn day(&self) -> u8 {
        04
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let paper = parse_input(raw).paper;

        let mut counts = vec![vec![0; paper.x()]; paper.y()];

        for x in 0..paper.x() {
            for y in 0..paper.y() {
                let pos = Pos2U(x, y);
                for dir in Dir8::all() {
                    if let Some(ch) = paper.get_at_posi(&pos.add(dir.dir())) {
                        if ch.eq(&'@') {
                            counts[y][x] += 1;
                        }
                    }
                }
            }
        }

        let mut count = 0;
        for y in 0..counts.len() {
            for x in 0..counts[y].len() {
                if counts[y][x] < 4 && paper.get(x, y).unwrap().eq(&'@') {
                    count += 1;
                }
            }
        }
        println!("Answer is {}", count);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let mut paper = parse_input(raw).paper;

        let mut removed = 0;
        let mut changed = true;
        while changed {
            let mut next_paper = paper.clone();
            changed = false;
            for x in 0..paper.x() {
                for y in 0..paper.y() {
                    if paper.get(x, y).unwrap().eq(&'@')
                        && (Dir8::all()
                            .iter()
                            .map(|dir| match paper.get_at_posi(&Pos2U(x, y).add(dir.dir())) {
                                Some('@') => 1,
                                _ => 0,
                            })
                            .sum::<usize>()
                            < 4)
                    {
                        next_paper.0[y][x] = '.';
                        removed += 1;
                        changed = true;
                    }
                }
            }
            paper = next_paper;
        }

        println!("Answer is {}", removed);
    }
}
