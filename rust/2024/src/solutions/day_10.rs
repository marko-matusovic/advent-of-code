use std::collections::HashMap;
use std::{collections::HashSet, ops::Add};

use itertools::Itertools;

use crate::libs::dir_2d::Dir4;
use crate::libs::{dir_2d, Grid2, Pos2U};

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    grid: Grid2<u8>,
}

type StepGraph = HashMap<u8, HashMap<Pos2U, HashSet<Pos2U>>>;

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let grid = lines
        .iter()
        .map(|l| l.chars().map(|ch| ch as u8 - '0' as u8).collect_vec())
        .collect_vec();
    Input {
        lines,
        grid: Grid2(grid),
    }
}

pub struct Day10;
impl Day for Day10 {
    fn day(&self) -> u8 {
        10
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let grid = input.grid.0;

        let mut score = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let mut pos = HashSet::new();
                if grid[y][x] == 0 {
                    pos.insert(Pos2U(x, y));
                    for step in 1..=9 {
                        let mut next_pos = HashSet::new();
                        for p in pos {
                            for dir in dir_2d::Dir4::all() {
                                if let Ok(Pos2U(x, y)) = p.add(dir.dir()).try_into() {
                                    if let Some(Some(&height)) = grid.get(y).map(|line| line.get(x))
                                    {
                                        if step == height {
                                            next_pos.insert(Pos2U(x, y));
                                        }
                                    }
                                }
                            }
                        }
                        pos = next_pos;
                    }
                }
                score += pos.len();
            }
        }

        println!("Answer is {}", score);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let mut score = 0;
        for y in 0..input.grid.0.len() {
            for x in 0..input.grid.0[y].len() {
                let pos = Pos2U(x, y);
                if let Some(&height) = input.grid.get_at(&pos) {
                    if height == 0 {
                        score += count_paths(&input.grid, 0, &pos)
                    }
                }
            }
        }

        println!("Answer is {}", score);
    }
}

fn count_paths(grid: &Grid2<u8>, step: u8, pos: &Pos2U) -> usize {
    if step == 9 {
        return 1;
    }

    let mut combinations = 0;
    for dir in Dir4::all() {
        if let Ok(next) = pos.add(dir.dir()).try_into() {
            if let Some(&height) = grid.get_at(&next) {
                if height == step + 1 {
                    combinations += count_paths(grid, step + 1, &next)
                }
            }
        }
    }

    combinations
}
