use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::libs::{Grid2, Pos2U, UnionFind};

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    grid: Grid2<char>,
    areas: HashSet<char>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let areas: HashSet<char> = lines.iter().flat_map(|l| l.chars()).collect();
    Input {
        grid: Grid2::from(&lines),
        areas,
        lines,
    }
}

pub struct Day12;
impl Day for Day12 {
    fn day(&self) -> u8 {
        12
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let mut uf = UnionFind::from(
            (0..input.grid.y())
                .flat_map(|y| (0..input.grid.x()).map(|x| Pos2U(x, y)).collect_vec())
                .collect_vec(),
        );

        let mut touching_neighbours: HashMap<Pos2U, usize> = HashMap::new();

        for y in 0..input.grid.y() {
            for x in 0..input.grid.x() {
                let pos = Pos2U(x, y);
                let ch = input.grid.get_at(&pos).unwrap();
                let right = Pos2U(x + 1, y);
                let down = Pos2U(x, y + 1);
                if let Some(nch) = input.grid.get_at(&right) {
                    if ch == nch {
                        uf.union(&pos, &right);
                        touching_neighbours
                            .entry(pos)
                            .and_modify(|c| {
                                *c += 1;
                            })
                            .or_insert(1);
                        touching_neighbours
                            .entry(right)
                            .and_modify(|c| {
                                *c += 1;
                            })
                            .or_insert(1);
                    }
                }
                if let Some(nch) = input.grid.get_at(&down) {
                    if ch == nch {
                        uf.union(&pos, &down);
                        touching_neighbours
                            .entry(pos)
                            .and_modify(|c| {
                                *c += 1;
                            })
                            .or_insert(1);
                        touching_neighbours
                            .entry(down)
                            .and_modify(|c| {
                                *c += 1;
                            })
                            .or_insert(1);
                    }
                }
            }
        }

        let value: usize = uf
            .clusters()
            .iter()
            .map(|cluster: &Vec<Pos2U>| {
                let area = cluster.len();
                let perimeter: usize = cluster
                    .iter()
                    .map(|p| 4 - touching_neighbours.get(p).unwrap_or(&0))
                    .sum();
                area * perimeter
            })
            .sum();

        println!("Answer is {}", value);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let mut uf = UnionFind::from(
            (0..input.grid.y())
                .flat_map(|y| (0..input.grid.x()).map(|x| Pos2U(x, y)).collect_vec())
                .collect_vec(),
        );

        for y in 0..input.grid.y() {
            for x in 0..input.grid.x() {
                let pos = Pos2U(x, y);
                let ch = input.grid.get_at(&pos).unwrap();
                let right = Pos2U(x + 1, y);
                let down = Pos2U(x, y + 1);
                if let Some(nch) = input.grid.get_at(&right) {
                    if ch == nch {
                        uf.union(&pos, &right);
                    }
                }
                if let Some(nch) = input.grid.get_at(&down) {
                    if ch == nch {
                        uf.union(&pos, &down);
                    }
                }
            }
        }

        let mut sides: HashMap<usize, usize> = HashMap::new();
        let mut grid = input.grid.clone();
        for i in 0..4 {
            grid = grid.rotate_ccw();
            for y in 0..grid.y() {
                for x in 0..grid.x() {
                    let orig = rotate_cw(&Pos2U(x, y), grid.x(), grid.y(), i);
                    let ch = *grid.get(x, y).unwrap();
                    // H is HERE
                    // X == group and must be present
                    // O != group and can also be out of bound

                    // HO
                    // XX
                    let is_s1 = true
                        && grid.get(x + 1, y).map_or(false, |&nch| ch != nch)
                        && grid.get(x, y + 1).map_or(false, |&nch| ch == nch)
                        && grid.get(x + 1, y + 1).map_or(false, |&nch| ch == nch);
                    // HO
                    // O
                    let is_s2 = true
                        && grid.get(x + 1, y).map_or(true, |&nch| ch != nch)
                        && grid.get(x, y + 1).map_or(true, |&nch| ch != nch);

                    if is_s1 || is_s2 {
                        // println!("{} @ ({},{})->{} i:{}", ch, x, y, orig.to_string(), i);
                        sides
                            .entry(uf.find(&orig))
                            .and_modify(|c| {
                                *c += 1;
                            })
                            .or_insert(1);
                    }
                }
            }
        }

        let value: usize = uf
            .clusters()
            .iter()
            .map(|cluster: &Vec<Pos2U>| {
                let area = cluster.len();
                let sides = *sides.get(&uf.find(cluster.first().unwrap())).unwrap();

                println!(
                    "{} a{} s{}",
                    grid.get_at(cluster.first().unwrap()).unwrap(),
                    &area,
                    &sides
                );

                area * sides
            })
            .sum();

        println!("Answer is {}", value);
    }
}

fn rotate_cw(pos: &Pos2U, x: usize, y: usize, n: usize) -> Pos2U {
    if n % 4 == 0 {
        return pos.to_owned();
    }
    rotate_cw(&Pos2U(y - pos.1 - 1, pos.0), y, x, (n % 4) - 1)
}
