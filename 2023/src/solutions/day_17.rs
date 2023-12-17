use std::collections::{BinaryHeap, HashSet};

use crate::libs::{
    dir::{Dir4, Rotate},
    pos_2i::Pos2I,
    pos_2u::Pos2U,
};

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    grid: Vec<Vec<usize>>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let grid = lines
        .iter()
        .map(|l| l.chars().map(|ch| ch as usize - '0' as usize).collect())
        .collect();
    Input { lines, grid }
}

pub struct Day17;
impl Day for Day17 {
    fn day(&self) -> u8 {
        17
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let min_heat = find_min_heat(
            &input.grid,
            vec![
                State(0, Pos2I(1, 0), Dir4::E, 1),
                State(0, Pos2I(0, 1), Dir4::S, 1),
            ],
            &Pos2U(input.grid[0].len() - 1, input.grid.len() - 1),
            0,
            3,
        );

        println!("Answer is {}", min_heat);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let min_heat = find_min_heat(
            &input.grid,
            vec![
                State(0, Pos2I(1, 0), Dir4::E, 1),
                State(0, Pos2I(0, 1), Dir4::S, 1),
            ],
            &Pos2U(input.grid[0].len() - 1, input.grid.len() - 1),
            4,
            10,
        );

        println!("Answer is {}", min_heat);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State(usize, Pos2I, Dir4, usize);

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let rank = |s: &Self| s.0 as isize - (s.1 .0 + s.1 .1) as isize;
        Some(rank(&self).cmp(&rank(&other)).reverse())
        // Some(self.0.cmp(&other.0).reverse())
    }
}

fn find_min_heat(
    grid: &Vec<Vec<usize>>,
    init_states: Vec<State>,
    goal: &Pos2U,
    min: usize,
    max: usize,
) -> usize {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut explored: HashSet<State> = HashSet::new();

    queue.extend(init_states);

    while let Some(state) = queue.pop() {
        let State(cur_heat, cur_pos, cur_dir, cur_line) = state;

        // Check if already explored
        let cache_state = State(0, cur_pos, cur_dir, cur_line);
        if explored.contains(&cache_state) {
            continue;
        }
        explored.insert(cache_state);

        // Check valid coords
        let res: Result<Pos2U, _> = cur_pos.try_into();
        if res.is_err() {
            continue;
        }
        let cur_pos = res.unwrap();
        if cur_pos.0 >= grid[0].len() || cur_pos.1 >= grid.len() {
            continue;
        }

        let moment_heat = grid[cur_pos.1][cur_pos.0];

        if min <= cur_line && cur_pos == *goal {
            return cur_heat + moment_heat;
        }

        // Find next moves
        if cur_line < max {
            queue.push(State(
                cur_heat + moment_heat,
                cur_pos + cur_dir.dir(),
                cur_dir,
                cur_line + 1,
            ));
        }
        if min <= cur_line {
            queue.push(State(
                cur_heat + moment_heat,
                cur_pos + cur_dir.rotate(90).dir(),
                cur_dir.rotate(90),
                1,
            ));
            queue.push(State(
                cur_heat + moment_heat,
                cur_pos + cur_dir.rotate(-90).dir(),
                cur_dir.rotate(-90),
                1,
            ));
        }
    }

    panic!()
}
