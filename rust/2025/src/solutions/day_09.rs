use super::day_trait::Day;
use crate::libs::dir_2d::Dir4;
use crate::libs::Pos2U;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::VecDeque;
use std::iter::once;

#[derive(Debug)]
pub struct Input {
    tiles: Vec<Pos2U>,
}

fn parse_input(raw: &str) -> Input {
    Input {
        tiles: raw
            .lines()
            .map(|l| {
                l.split(",")
                    .map(|num| num.parse().unwrap())
                    .collect::<Pos2U>()
            })
            .collect(),
    }
}

pub struct Day09;
impl Day for Day09 {
    fn day(&self) -> u8 {
        09
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let tiles = parse_input(raw).tiles;

        let size = tiles
            .iter()
            .tuple_combinations()
            .map(|(t1, t2)| (t1.dist_x(t2) + 1) * (t1.dist_y(t2) + 1))
            .max()
            .unwrap();

        println!("Answer is {}", size);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let tiles = parse_input(raw).tiles;

        let MinMax(min_x, max_x) = tiles.iter().map(|t| t.0).minmax() else {
            panic!()
        };
        let MinMax(min_y, max_y) = tiles.iter().map(|t| t.1).minmax() else {
            panic!()
        };

        let x_value: Vec<usize> = tiles
            .iter()
            .map(|t| t.0)
            .chain(once(min_x - 1))
            .chain(once(max_x + 1))
            .unique()
            .sorted()
            .collect();
        let y_value: Vec<usize> = tiles
            .iter()
            .map(|t| t.1)
            .chain(once(min_y - 1))
            .chain(once(max_y + 1))
            .unique()
            .sorted()
            .collect();

        let x_index = |x| x_value.iter().position(|&v| v == x).unwrap();
        let y_index = |y| y_value.iter().position(|&v| v == y).unwrap();

        let mut grid: Vec<Vec<bool>> = vec![vec![false; y_value.len()]; x_value.len()];
        for (t1, t2) in tiles.iter().tuple_windows() {
            let x_min = t1.0.min(t2.0);
            let x_max = t1.0.max(t2.0);
            let y_min = t1.1.min(t2.1);
            let y_max = t1.1.max(t2.1);
            let xi = x_index(x_min);
            let yi = y_index(y_min);
            for yi in y_index(y_min)..=y_index(y_max) {
                grid[yi][xi] = true;
            }
            for xi in x_index(x_min)..=x_index(x_max) {
                grid[yi][xi] = true;
            }
        }
        let walls = grid.clone();
        let mut outside = grid.clone();
        let mut queue = VecDeque::new();
        queue.push_back(Pos2U(0, 0));
        while !queue.is_empty() {
            if let Some(pos) = queue.pop_front() {
                if outside.len() <= pos.0 || outside[pos.0].len() <= pos.1 || outside[pos.0][pos.1]
                {
                    continue;
                }
                outside[pos.0][pos.1] = true;
                for dir in Dir4::all() {
                    if let Ok(pos) = (pos + dir.dir()).try_into() {
                        queue.push_back(pos);
                    }
                }
            }
        }

        let mut inside = walls.clone();
        for y in 0..outside.len() {
            for x in 0..outside[y].len() {
                if !outside[y][x] {
                    inside[y][x] = true;
                }
            }
        }

        // // DEBUG PRINT
        // for row in inside.clone() {
        //     for cell in row {
        //         print!("{}", if cell { "#" } else { "." });
        //     }
        //     println!()
        // }

        let size = tiles
            .iter()
            .tuple_combinations()
            .filter(|&(t1, t2)| {
                let x_min = t1.0.min(t2.0);
                let x_max = t1.0.max(t2.0);
                let y_min = t1.1.min(t2.1);
                let y_max = t1.1.max(t2.1);
                for xi in x_index(x_min)..=x_index(x_max) {
                    for yi in y_index(y_min)..=y_index(y_max) {
                        if !inside[yi][xi] {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|(t1, t2)| (t1.dist_x(t2) + 1) * (t1.dist_y(t2) + 1))
            .max()
            .unwrap();

        println!("Answer is {}", size);
    }
}
