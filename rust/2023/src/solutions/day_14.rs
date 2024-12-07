use std::collections::HashMap;

use super::day_trait::Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rock {
    Air,
    Square,
    Round,
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            '.' => Rock::Air,
            '#' => Rock::Square,
            'O' => Rock::Round,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Grid(Vec<Vec<Rock>>);

impl Grid {
    fn shift_north(&self) -> Self {
        let mut next: Vec<Vec<Rock>> = self.0.clone();
        for c in 0..next[0].len() {
            for r in 0..next.len() {
                if next[r][c] == Rock::Round {
                    let mut i = 0;
                    while i < r && next[r-i-1][c] == Rock::Air {
                        i += 1;
                    }
                    next[r][c] = Rock::Air;
                    next[r-i][c] = Rock::Round;
                }
            }
        }

        Grid(next)
    }
    fn shift_west(&self) -> Self {
        let mut next: Vec<Vec<Rock>> = self.0.clone();
        for c in 0..next[0].len() {
            for r in 0..next.len() {
                if next[r][c] == Rock::Round {
                    let mut i = 0;
                    while i < c && next[r][c-i-1] == Rock::Air {
                        i += 1;
                    }
                    next[r][c] = Rock::Air;
                    next[r][c-i] = Rock::Round;
                }
            }
        }

        Grid(next)
    }
    fn shift_south(&self) -> Self {
        let mut next: Vec<Vec<Rock>> = self.0.clone();
        for c in 0..next[0].len() {
            for r in 0..next.len() {
                let r = next.len() - r - 1;
                if next[r][c] == Rock::Round {
                    let mut i = 0;
                    while r + i + 1 < next.len() && next[r+i+1][c] == Rock::Air {
                        i += 1;
                    }
                    next[r][c] = Rock::Air;
                    next[r+i][c] = Rock::Round;
                }
            }
        }

        Grid(next)
    }
    fn shift_east(&self) -> Self {
        let mut next: Vec<Vec<Rock>> = self.0.clone();
        for c in 0..next[0].len() {
            let c = next[0].len() - c - 1;
            for r in 0..next.len() {
                if next[r][c] == Rock::Round {
                    let mut i = 0;
                    while c + i + 1 < next[0].len() && next[r][c+i+1] == Rock::Air {
                        i += 1;
                    }
                    next[r][c] = Rock::Air;
                    next[r][c+i] = Rock::Round;
                }
            }
        }

        Grid(next)
    }

    fn score(&self) -> usize {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| (i + 1) * row.iter().filter(|rk| **rk == Rock::Round).count())
            .sum()
    }

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Input {
    lines: Vec<String>,
    grid: Grid,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();

    let grid = Grid(
        lines
            .iter()
            .map(|l| l.chars().map(Rock::from).collect())
            .collect(),
    );

    Input { lines, grid }
}

pub struct Day14;
impl Day for Day14 {
    fn day(&self) -> u8 {
        14
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let shifted = input.grid.shift_north();

        println!("Answer is {}", shifted.score());
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let mut grid = input.grid.clone();

        let mut cycles: HashMap<Grid, usize> = HashMap::new();

        let mut cycle = 0;
        while cycle < 1000000000 {
            grid = grid.shift_north().shift_west().shift_south().shift_east();
            cycle += 1;
            if let Some(old_cycle) = cycles.get(&grid) {
                let cyc_len = cycle - old_cycle;
                cycle += ((1000000000 - cycle) / cyc_len) * cyc_len;
            } else {
                cycles.insert(grid.clone(), cycle);
            }
        }

        println!("Answer is {}", grid.score());
    }
}
