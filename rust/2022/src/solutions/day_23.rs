use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::Add,
};

use itertools::Itertools;

pub fn day() -> u8 {
    23
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(isize, isize);
impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Heading {
    E,
    SE,
    S,
    SW,
    W,
    NW,
    N,
    NE,
}
impl Heading {
    fn dir(&self) -> Pos {
        match self {
            Heading::E => Pos(1, 0),
            Heading::SE => Pos(1, 1),
            Heading::S => Pos(0, 1),
            Heading::SW => Pos(-1, 1),
            Heading::W => Pos(-1, 0),
            Heading::NW => Pos(-1, -1),
            Heading::N => Pos(0, -1),
            Heading::NE => Pos(1, -1),
        }
    }
    fn checks(&self) -> [Heading; 3] {
        match self {
            Heading::E => [Heading::E, Heading::NE, Heading::SE],
            Heading::S => [Heading::S, Heading::SW, Heading::SE],
            Heading::W => [Heading::W, Heading::NW, Heading::SW],
            Heading::N => [Heading::N, Heading::NW, Heading::NE],
            _ => panic!(),
        }
    }
    fn all() -> Vec<Heading> {
        vec![
            Heading::E,
            Heading::SE,
            Heading::S,
            Heading::SW,
            Heading::W,
            Heading::NW,
            Heading::N,
            Heading::NE,
        ]
    }
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    elves: HashSet<Pos>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());

    let elves = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, ch)| match ch {
                '#' => Some(Pos(x.to_owned() as isize, y.to_owned() as isize)),
                _ => None,
            })
        })
        .collect();

    Input { lines, elves }
}

fn elf_will_move(elf: &Pos, gang: &HashSet<Pos>) -> bool {
    Heading::all()
        .iter()
        .map(|h| *elf + h.dir())
        .any(|p| gang.contains(&p))
}

fn propose_elf(elf: &Pos, consider: &Vec<Heading>, gang: &HashSet<Pos>) -> Option<Heading> {
    if !elf_will_move(elf, gang) {
        return None;
    }

    consider
        .iter()
        .find(|heading| {
            heading
                .checks()
                .iter()
                .all(|check| !gang.contains(&(*elf + check.dir())))
        })
        .map(|h| h.to_owned())
}

fn all_elves_step(elves: &HashSet<Pos>, consider: &Vec<Heading>) -> HashSet<Pos> {
    // Part 1 - propose
    let proposals: Vec<(Pos, Option<Heading>)> = elves
        .iter()
        .map(|e| (*e, propose_elf(e, consider, elves)))
        .collect();
    let all_dests: Vec<Pos> = proposals
        .iter()
        .filter(|(_, h)| h.is_some())
        .map(|(e, h)| *e + h.unwrap().dir())
        .collect();
    let legit: HashSet<Pos> = all_dests
        .iter()
        .unique()
        .filter(|d| all_dests.iter().filter(|ad| ad == d).count() == 1)
        .map(Pos::to_owned)
        .collect();
    // Part 2 - step
    return proposals
        .iter()
        .map(|(e, h)| {
            if h.is_some() && legit.contains(&(*e + h.unwrap().dir())) {
                *e + h.unwrap().dir()
            } else {
                e.to_owned().to_owned()
            }
        })
        .collect();
}

fn bounds(elves: &HashSet<Pos>) -> (Pos, Pos) {
    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;

    for elf in elves {
        min_x = min(min_x, elf.0);
        min_y = min(min_y, elf.1);
        max_x = max(max_x, elf.0);
        max_y = max(max_y, elf.1);
    }

    (Pos(min_x, min_y), Pos(max_x, max_y))
}

fn area(elves: &HashSet<Pos>) -> usize {
    let (Pos(min_x, min_y), Pos(max_x, max_y)) = bounds(elves);
    (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize
}

fn print_elves(elves: &HashSet<Pos>) {
    let (Pos(min_x, min_y), Pos(max_x, max_y)) = bounds(elves);

    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            if elves.iter().any(|e| *e == Pos(x, y)) {
                print!("#");
            } else {
                print!(".");
            };
        }
        println!();
    }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let mut elves = input.elves.clone();
    let mut consider = vec![Heading::N, Heading::S, Heading::W, Heading::E];
    for _ in 0..10 {
        elves = all_elves_step(&elves, &consider);
        consider.rotate_left(1);
    }

    let total_area = area(&elves);

    println!("Answer is {}", total_area - elves.len());
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let mut elves = input.elves.clone();
    let mut consider = vec![Heading::N, Heading::S, Heading::W, Heading::E];
    let mut round = 1;
    while elves.iter().any(|e| elf_will_move(e, &elves)) {
        elves = all_elves_step(&elves, &consider);
        consider.rotate_left(1);
        round += 1;
    }

    println!("Answer is {}", round);
}
