use std::ops::{Add, Sub};

use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos2D(usize, usize);

impl Add for Pos2D {
    type Output = Pos2D;

    fn add(self, other: Self) -> Self::Output {
        Pos2D(self.0 + other.0, self.1 + other.1)
    }
}
impl Sub for Pos2D {
    type Output = Pos2D;

    fn sub(self, other: Self) -> Self::Output {
        Pos2D(self.0 - other.0, self.1 - other.1)
    }
}
impl Pos2D {
    fn scale(self, sc: usize) -> Self {
        Pos2D(self.0 * sc, self.1 * sc)
    }
    fn dominates(&self, other: Self) -> bool {
        return other.0 <= self.0 && other.1 <= self.1;
    }
    // fn dist_n1(&self, other: Self) -> usize {
    //     ((self.0 as isize - other.0 as isize).abs() + (self.1 as isize - other.1 as isize).abs())
    //         as usize
    // }
    fn dist_n1(&self, other: Self) -> usize {
        let dx = if self.0 > other.0 {
            self.0 - other.0
        } else {
            other.0 - self.0
        };
        let dy = if self.1 > other.1 {
            self.1 - other.1
        } else {
            other.1 - self.1
        };
        dx + dy
    }
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    galaxies: Vec<Pos2D>,
    free_rows: Vec<usize>,
    free_cols: Vec<usize>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let galaxies: Vec<Pos2D> = lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .into_iter()
                .enumerate()
                .filter(|(_, s)| *s == '#')
                .map(|(x, _)| Pos2D(x as usize, y as usize))
                .collect_vec()
        })
        .filter(|vec| vec.len() > 0)
        .flatten()
        .collect();

    let free_rows: Vec<usize> = (0..lines.len())
        .into_iter()
        .filter(|row| !galaxies.iter().any(|Pos2D(_, y)| *y == *row as usize))
        .collect();
    let free_cols: Vec<usize> = (0..lines[0].len())
        .into_iter()
        .filter(|col| !galaxies.iter().any(|Pos2D(x, _)| *x == *col as usize))
        .collect();

    Input {
        lines,
        galaxies,
        free_rows,
        free_cols,
    }
}

pub struct Day11;
impl Day for Day11 {
    fn day(&self) -> u8 {
        11
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let expanded_galaxies = input
            .galaxies
            .iter()
            .map(|Pos2D(x, y)| {
                Pos2D(
                    x + input.free_cols.iter().take_while(|&c| c < x).count(),
                    y + input.free_rows.iter().take_while(|&r| r < y).count(),
                )
            })
            .collect_vec();

        let sum: usize = expanded_galaxies
            .iter()
            .combinations(2)
            .map(|combi| combi[0].dist_n1(combi[1].to_owned()))
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let expanded_galaxies = input
            .galaxies
            .iter()
            .map(|Pos2D(x, y)| {
                Pos2D(
                    x + (1000000 - 1) * input.free_cols.iter().take_while(|&c| c <= x).count(),
                    y + (1000000 - 1) * input.free_rows.iter().take_while(|&r| r <= y).count(),
                )
            })
            .collect_vec();

        let sum: usize = expanded_galaxies
            .iter()
            .combinations(2)
            .map(|combi| combi[0].dist_n1(combi[1].to_owned()))
            .sum();

        println!("Answer is {}", sum);
    }
}
