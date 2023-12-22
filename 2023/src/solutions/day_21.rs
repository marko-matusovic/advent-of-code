use std::collections::{BinaryHeap, HashSet};

use crate::libs::{Dir4, Pos2I, Pos2U};

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    rocks: HashSet<Pos2I>,
    start: Pos2I,
    grid_size: Pos2U,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();

    let h = lines.len();
    let w = lines[0].len();

    let rocks = lines
        .iter()
        .enumerate()
        .map(move |(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(x, _)| Pos2U(x, y).into())
        })
        .flatten()
        .collect();
    let start = lines
        .iter()
        .enumerate()
        .flat_map(move |(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == 'S')
                .map(move |(x, _)| Pos2U(x, y).into())
        })
        .find(|_| true)
        .unwrap();
    Input {
        lines,
        rocks,
        start,
        grid_size: Pos2U(w, h),
    }
}

pub struct Day21;
impl Day for Day21 {
    fn day(&self) -> u8 {
        21
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let w = input.grid_size.0;
        let h = input.grid_size.1;
        let rocks: HashSet<Pos2I> = input
            .rocks
            .iter()
            .map(|Pos2I(x, y)| Pos2I(x + 1, y + 1))
            .chain((0..=w + 1).into_iter().map(|x| Pos2U(x, 0).into()))
            .chain((0..=w + 1).into_iter().map(|x| Pos2U(x, h + 1).into()))
            .chain((0..=h + 1).into_iter().map(|y| Pos2U(0, y).into()))
            .chain((0..=h + 1).into_iter().map(|y| Pos2U(w + 1, y).into()))
            .collect();

        let count = count_possible_after(
            &(input.grid_size + Pos2U(1, 1)),
            &(input.start + Pos2I(1, 1)),
            &rocks,
            64,
        );

        println!("Answer is {}", count);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        dbg!(input.grid_size);

        let count = count_possible_after(&input.grid_size, &input.start, &input.rocks, 26501365);

        println!("Answer is {}", count);
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State(Pos2I, usize);

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.1.cmp(&other.1).reverse())
    }
}

fn count_possible_after(
    grid_size: &Pos2U,
    start: &Pos2I,
    rocks: &HashSet<Pos2I>,
    steps: usize,
) -> usize {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut possible: HashSet<Pos2I> = HashSet::new();
    let mut explored: HashSet<Pos2I> = HashSet::new();

    queue.push(State(start.clone(), 0));

    while let Some(State(cur_pos, cur_step)) = queue.pop() {
        print!("step: {}\r", cur_step);
        if cur_step > steps {
            break;
        }
        let wrapped_pos = Pos2I(
            cur_pos.0 % grid_size.0 as isize,
            cur_pos.1 % grid_size.1 as isize,
        );
        if rocks.contains(&wrapped_pos) {
            continue;
        }
        if explored.contains(&cur_pos) {
            continue;
        }
        explored.insert(cur_pos.clone());
        if cur_step % 2 == steps % 2 {
            possible.insert(cur_pos.clone());
        }
        for dir in Dir4::all().iter() {
            queue.push(State(
                (cur_pos + dir.dir()).try_into().unwrap(),
                cur_step + 1,
            ));
        }
    }

    possible.len()
}
