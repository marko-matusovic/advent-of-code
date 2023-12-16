use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::libs::{dir::{Dir4, Rotate}, pos_2i::Pos2I, pos_2u::Pos2U};

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    grid: Vec<Vec<char>>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();

    let grid = lines.iter().map(|l| l.chars().collect()).collect();

    Input { lines, grid }
}

pub struct Day16;
impl Day for Day16 {
    fn day(&self) -> u8 {
        16
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let sum = count_energised(&input.grid, State(Pos2I(0,0), Dir4::E));

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let h = input.grid.len() as isize;
        let w = input.grid[0].len() as isize;

        let mut max = 0;
        for x in 0..w {
            let top = count_energised(&input.grid, State(Pos2I(x,0), Dir4::S));
            let btm = count_energised(&input.grid, State(Pos2I(x,h-1), Dir4::N));
            max = vec![max, top, btm].iter().max().unwrap().to_owned();
        }
        for y in 0..h {
            let lft = count_energised(&input.grid, State(Pos2I(0, y), Dir4::E));
            let rgh = count_energised(&input.grid, State(Pos2I(w-1, y), Dir4::W));
            max = vec![max, lft, rgh].iter().max().unwrap().to_owned();
        }

        println!("Answer is {}", max);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State(Pos2I, Dir4);

fn count_energised(grid: &Vec<Vec<char>>, init_state: State) -> usize {
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut explored: HashSet<State> = HashSet::new();

    queue.push_front(init_state);

    while let Some(state) = queue.pop_front() {
        let State(cur_pos, cur_dir) = state;
        // println!("pos: ({},{}) dir: {}", cur_pos.0, cur_pos.1, cur_dir.deg());

        // Check valid coords
        let res: Result<Pos2U, _> = cur_pos.try_into();
        if res.is_err() {
            continue;
        }
        let cur_pos = res.unwrap();
        if cur_pos.0 >= grid[0].len() || cur_pos.1 >= grid.len() {
            continue;
        }

        // Check if already explored
        if explored.contains(&state) {
            continue;
        }
        explored.insert(state);

        // Find next moves
        match grid[cur_pos.1][cur_pos.0] {
            '.' => {
                queue.push_back(State(cur_pos + cur_dir.dir(), cur_dir));
            }
            '-' => {
                if cur_dir.is_horizontal() {
                    queue.push_back(State(cur_pos + cur_dir.dir(), cur_dir));
                } else {
                    queue.push_back(State(cur_pos + Pos2I(1, 0), Dir4::E));
                    queue.push_back(State(cur_pos + Pos2I(-1, 0), Dir4::W));
                }
            }
            '|' => {
                if cur_dir.is_vertical() {
                    queue.push_back(State(cur_pos + cur_dir.dir(), cur_dir));
                } else {
                    queue.push_back(State(cur_pos + Pos2I(0, 1), Dir4::S));
                    queue.push_back(State(cur_pos + Pos2I(0, -1), Dir4::N));
                }
            }
            '/' => {
                let next_dir = match cur_dir.is_horizontal() {
                    true => cur_dir.rotate(-90),
                    false => cur_dir.rotate(90),
                };
                queue.push_back(State(cur_pos + next_dir.dir(), next_dir));
            },
            '\\' => {
                let next_dir = match cur_dir.is_horizontal() {
                    true => cur_dir.rotate(90),
                    false => cur_dir.rotate(-90),
                };
                queue.push_back(State(cur_pos + next_dir.dir(), next_dir));
            },
            _ => panic!(),
        }
    }

    explored.iter().map(|State(pos, _)| pos.to_owned()).unique().count()
}
