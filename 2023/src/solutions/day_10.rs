use std::ops::{Add, Sub};   

use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos2D(isize, isize);

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
    fn scale(self, sc: isize) -> Self {
        Pos2D(self.0 * sc, self.1 * sc)
    }
    fn dominates(&self, other: Self) -> bool {
        return other.0 <= self.0 && other.1 <= self.1;
    }
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    map: Vec<Vec<char>>,
}

impl Input {
    fn start_pos(&self) -> Pos2D {
        for (i, row) in self.map.iter().enumerate() {
            for (j, &item) in row.iter().enumerate() {
                if item == 'S' {
                    return Pos2D(j as isize, i as isize);
                }
            }
        }
        panic!()
    }
    fn at_map(&self, pos: Pos2D) -> Option<char> {
        if pos.dominates(Pos2D(0, 0))
            && Pos2D(self.map[0].len() as isize, self.map.len() as isize).dominates(pos)
        {
            return Some(self.map[pos.1 as usize][pos.0 as usize]);
        }
        return None;
    }

    fn trace_next(&self, cur: Pos2D, from_dir: Pos2D) -> (Pos2D, Pos2D) {
        let cur_char = self.at_map(cur).unwrap();
        match pipe_dirs(&cur_char)
            .iter()
            .find(|&d| *d != from_dir.scale(-1))
        {
            Some(&next_dir) => (cur + next_dir, next_dir),
            None => panic!(),
        }
    }

    fn start_pos_dir(&self) -> (Pos2D, Pos2D) {
        let start = self.start_pos();
        let mut dir: Pos2D = Pos2D(0,0);
        for d in vec![Pos2D(1, 0), Pos2D(0, 1), Pos2D(-1, 0), Pos2D(0, -1)] {
            if let Some(p) = self.at_map(start + d) {
                if pipe_dirs(&p).iter().any(|&d2| d2 == d.scale(-1)) {
                    dir = d;
                    break;
                }
            }
        }
        if dir == Pos2D(0,0) {
            panic!();
        }
        return (start + dir, dir);
    }
}

fn pipe_dirs(pipe: &char) -> Vec<Pos2D> {
    match pipe {
        '-' => vec![Pos2D(1, 0), Pos2D(-1, 0)],
        '|' => vec![Pos2D(0, 1), Pos2D(0, -1)],
        '7' => vec![Pos2D(-1, 0), Pos2D(0, 1)],
        'J' => vec![Pos2D(-1, 0), Pos2D(0, -1)],
        'L' => vec![Pos2D(1, 0), Pos2D(0, -1)],
        'F' => vec![Pos2D(1, 0), Pos2D(0, 1)],
        _ => panic!(),
    }
}

fn shoelace_area(points: &Vec<Pos2D>) -> f64 {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        sum1 += points[i].0 * points[j].1;
        sum2 += points[j].0 * points[i].1;
    }

    ((sum1 - sum2) as f64).abs() / 2.0
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let map = lines.iter().map(|l| l.chars().collect_vec()).collect_vec();

    Input { lines, map }
}

pub struct Day10;
impl Day for Day10 {
    fn day(&self) -> u8 {
        10
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let start_pos_dir = input.start_pos_dir();
        let mut cur = start_pos_dir.0;
        let mut dir = start_pos_dir.1;
        let mut loop_size = 1;

        while input.at_map(cur) != Some('S') {
            (cur, dir) = input.trace_next(cur, dir);
            loop_size += 1;
        }

        println!("Answer is {}", loop_size / 2);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let start_pos_dir = input.start_pos_dir();
        let mut cur = start_pos_dir.0;
        let mut dir = start_pos_dir.1;

        let mut pipes: Vec<Pos2D> = vec![cur.clone()];

        while input.at_map(cur) != Some('S') {
            (cur, dir) = input.trace_next(cur, dir);
            pipes.push(cur.clone());
        }

        let area = shoelace_area(&pipes);
        let empty_area = ((pipes.len() / 2) - 1) as f64;
        
        println!("Answer is {}", area - empty_area);
    }
}
