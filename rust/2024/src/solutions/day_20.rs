use std::{
    collections::{BinaryHeap, HashSet},
    ops::Add,
};

use itertools::Itertools;

use crate::libs::{dir_2d::Dir4, Pos2U};

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    obstacles: HashSet<Pos2U>,
    start: Pos2U,
    end: Pos2U,
}

fn parse_input(raw: &str) -> Input {
    let mut obstacles = HashSet::new();
    let mut start: Option<Pos2U> = None;
    let mut end: Option<Pos2U> = None;
    for (y, line) in raw.split("\n").enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    obstacles.insert(Pos2U(x, y));
                }
                'S' => {
                    start = Some(Pos2U(x, y));
                }
                'E' => {
                    end = Some(Pos2U(x, y));
                }
                '.' => {}
                _ => panic!("Invalid input {} at x:{} y:{}", ch, x, y),
            }
        }
    }
    Input {
        obstacles,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    pos: Pos2U,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day20;
impl Day for Day20 {
    fn day(&self) -> u8 {
        20
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let min_cost = least_cost(&input.obstacles, &input.start, &input.end).unwrap();

        let count = all_routes(&input.obstacles, &input.start, &input.end, 1, min_cost - 100)
            .iter()
            .count();

        println!("Answer is {}", count);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let _input: Input = parse_input(raw);

        println!("Answer is {}", 0);
    }
}

fn least_cost(obstacles: &HashSet<Pos2U>, start: &Pos2U, end: &Pos2U) -> Option<usize> {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashSet<Pos2U> = HashSet::new();
    queue.push(State {
        pos: start.clone(),
        cost: 0,
    });

    while let Some(State { pos, cost }) = queue.pop() {
        if !visited.insert(pos.clone()) {
            continue;
        }

        if pos == *end {
            return Some(cost);
        }

        for dir in Dir4::all() {
            if let Ok(next) = TryInto::<Pos2U>::try_into(pos.add(dir.dir())) {
                if !obstacles.contains(&next) {
                    queue.push(State {
                        pos: next,
                        cost: cost + 1,
                    });
                }
            }
        }
    }

    None
}

fn all_routes(
    obstacles: &HashSet<Pos2U>,
    start: &Pos2U,
    end: &Pos2U,
    cheating: usize,
    max_cost: usize
) -> Vec<Vec<Pos2U>> {
    all_routes_rec(obstacles, &HashSet::new(), start, end, cheating, max_cost).unwrap_or_else(|| Vec::new())
}

fn all_routes_rec(
    obstacles: &HashSet<Pos2U>,
    visited: &HashSet<Pos2U>,
    current: &Pos2U,
    end: &Pos2U,
    cheating: usize,
    max_cost: usize
) -> Option<Vec<Vec<Pos2U>>> {
    if current == end {
        return Some(vec![vec![end.clone()]]);
    }

    print!("{}, ", max_cost);
    if max_cost <= 0 {
        return None;
    }

    let mut visited = visited.clone();
    if !visited.insert(current.clone()) {
        return None;
    }

    Dir4::all()
        .iter()
        .map(|dir| {
            if let Ok(next) = TryInto::<Pos2U>::try_into(current.add(dir.dir())) {
                let routes = match (obstacles.contains(&next), cheating > 0) {
                    (false, _) => all_routes_rec(obstacles, &visited, &next, end, cheating, max_cost - 1),
                    (true, true) => all_routes_rec(obstacles, &visited, &next, end, cheating - 1, max_cost - 1),
                    (_, _) => None,
                };
                return routes.map(|routes| {
                    routes
                        .iter()
                        .map(|route| vec![vec![current.clone()], route.clone()].concat())
                        .collect_vec()
                });
            }
            None
        })
        .fold(None, |a, b| match (a, b) {
            (Some(mut x), Some(y)) => {
                x.extend(y);
                Some(x)
            }
            (a, None) => a,
            (None, b) => b,
        })
}
