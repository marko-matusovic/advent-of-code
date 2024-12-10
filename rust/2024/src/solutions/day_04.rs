use itertools::Itertools;

use crate::libs::Grid2;

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    grid: Grid2<char>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let grid = lines.iter().map(|l| l.chars().collect_vec()).collect_vec();
    Input {
        lines,
        grid: Grid2(grid)
    }
}

pub struct Day04;
impl Day for Day04 {
    fn day(&self) -> u8 {
        04
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let pattern = "XMAS";
        let mut grid = input.grid.clone();

        let mut count = 0;
        for _ in 0..4 {
            for y in 0..grid.0.len() {
                for x in 0..grid.0[y].len() {
                    if pattern.chars().into_iter().enumerate().all(|(i, ch)| {
                        if let Some(Some(&ch2)) = grid.0.get(y).map(|yr| yr.get(x + i)) {
                            ch == ch2
                        } else {
                            false
                        }
                    }) {
                        count += 1;
                    }
                    if pattern.chars().into_iter().enumerate().all(|(i, ch)| {
                        if let Some(Some(&ch2)) = grid.0.get(y + i).map(|yr| yr.get(x + i)) {
                            ch == ch2
                        } else {
                            false
                        }
                    }) {
                        count += 1;
                    }
                }
            }
            grid = grid.rotate_cw();
        }

        println!("Answer is {}", count);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let pattern = "MAS";
        let mut grid = input.grid.clone();

        let mut count = 0;
        for _ in 0..4 {
            for y in 0..grid.0.len() {
                for x in 0..grid.0[y].len() {
                    if pattern.chars().into_iter().enumerate().all(|(i, ch)| {
                        (if let Some(Some(&ch2)) = grid.0.get(y + i).map(|yr| yr.get(x + i)) {
                            ch == ch2
                        } else {
                            false
                        }) && (if let Some(Some(&ch2)) = grid.0
                            .get(y + i)
                            .map(|yr| yr.get(x + pattern.len() - i - 1))
                        {
                            ch == ch2
                        } else {
                            false
                        })
                    }) {
                        count += 1;
                    }
                }
            }
            grid = grid.rotate_cw();
        }

        println!("Answer is {}", count);
    }
}
