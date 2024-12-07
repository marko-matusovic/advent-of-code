use std::{collections::HashSet, ops::Add};

use crate::libs::{dir_2d::Dir4, Pos2I};

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    obstacles: HashSet<Pos2I>,
    start_pos: Pos2I,
    dim: Pos2I,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let mut obstacles: HashSet<Pos2I> = HashSet::new();
    let mut start_pos: Option<Pos2I> = None;
    let dim = Pos2I(lines[0].len() as isize, lines.len() as isize);
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            // if let Some('#') = line.chars().nth(x) {
            //     obstacles.insert(Pos2I(x, y));
            // }
            match lines[y].chars().nth(x) {
                Some('#') => {
                    obstacles.insert(Pos2I(x as isize, y as isize));
                }
                Some('^') => {
                    start_pos = Some(Pos2I(x as isize, y as isize));
                }
                _ => {}
            }
        }
    }
    if let Some(start_pos) = start_pos {
        return Input {
            lines,
            obstacles,
            start_pos,
            dim,
        };
    }
    panic!("No starting pos found!")
}

pub struct Day06;
impl Day for Day06 {
    fn day(&self) -> u8 {
        06
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let mut visited: HashSet<Pos2I> = HashSet::new();

        let mut cur = input.start_pos.clone();
        let mut dir = Dir4::N;

        while 0 <= cur.0 && cur.0 < input.dim.0 && 0 <= cur.1 && cur.1 < input.dim.1 {
            visited.insert(cur.clone());
            let next = cur.add(dir.dir());
            if input.obstacles.contains(&next) {
                dir = dir.rotate(90);
            } else {
                cur = next;
            }
        }

        println!("Answer is {}", visited.len());
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let mut count: usize = 0;
        for x in 0..input.dim.0 {
            for y in 0..input.dim.1 {
                if input.obstacles.contains(&Pos2I(x, y)) {
                    continue;
                }

                let mut cur = input.start_pos.clone();
                let mut dir = Dir4::N;
                let mut visited: HashSet<(Pos2I, Dir4)> = HashSet::new();
                let mut obstacles = input.obstacles.clone();
                obstacles.insert(Pos2I(x, y));

                while 0 <= cur.0 && cur.0 < input.dim.0 && 0 <= cur.1 && cur.1 < input.dim.1 {
                    let code = (cur.clone(), dir.clone());
                    if visited.contains(&code) {
                        count += 1;
                        break;
                    }
                    visited.insert(code);

                    let next = cur.add(dir.dir());
                    if obstacles.contains(&next) {
                        dir = dir.rotate(90);
                    } else {
                        cur = next;
                    }
                }
            }
        }

        println!("Answer is {}", count);
    }
}
