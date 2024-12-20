use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Add,
    usize,
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
    dir: Dir4,
    score: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day16;
impl Day for Day16 {
    fn day(&self) -> u8 {
        16
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let score = shortest_path(&input.obstacles, &input.start, &input.end).unwrap();

        println!("Answer is {}", score);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let count = shortest_path_2(&input.obstacles, &input.start, &input.end);

        println!("Answer is {}", count);
    }
}

fn shortest_path(obstacles: &HashSet<Pos2U>, start: &Pos2U, end: &Pos2U) -> Option<usize> {
    let mut visited: HashSet<(Pos2U, Dir4)> = HashSet::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        pos: start.clone(),
        dir: Dir4::E,
        score: 0,
    });
    while let Some(State { pos, dir, score }) = queue.pop() {
        if pos == *end {
            return Some(score);
        }

        if visited.contains(&(pos, dir)) {
            continue;
        }
        visited.insert((pos, dir));

        let next = pos.add(dir.dir()).try_into().unwrap();
        if !obstacles.contains(&next) {
            queue.push(State {
                pos: next,
                dir,
                score: score + 1,
            });
        }

        queue.push(State {
            pos,
            dir: dir.rotate(90),
            score: score + 1000,
        });

        queue.push(State {
            pos,
            dir: dir.rotate(-90),
            score: score + 1000,
        });
    }
    None
}

fn shortest_path_2(obstacles: &HashSet<Pos2U>, start: &Pos2U, end: &Pos2U) -> usize {
    let mut visited: HashSet<(Pos2U, Dir4)> = HashSet::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut best_previous: HashMap<(Pos2U, Dir4), Vec<(Pos2U, Dir4)>> = HashMap::new();
    let mut best_score: HashMap<(Pos2U, Dir4), usize> = HashMap::new();
    queue.push(State {
        pos: start.clone(),
        dir: Dir4::E,
        score: 0,
    });
    let mut winner_score = usize::MAX;
    while let Some(State { pos, dir, score }) = queue.pop() {
        if pos == *end {
            winner_score = score;
        }

        if visited.contains(&(pos, dir)) || score > winner_score {
            continue;
        }
        visited.insert((pos, dir));

        let next = pos.add(dir.dir()).try_into().unwrap();
        if !obstacles.contains(&next) {
            let key = (next, dir);
            if !best_score.contains_key(&key) || score + 1 < *best_score.get(&key).unwrap() {
                best_score.insert(key, score + 1);
                best_previous.insert(key, vec![(pos, dir)]);
            } else if score + 1 == *best_score.get(&key).unwrap() {
                best_previous.get_mut(&key).unwrap().push((pos, dir));
            }
            queue.push(State {
                pos: next,
                dir,
                score: score + 1,
            });
        }

        let key = (pos, dir.rotate(90));
        if !best_score.contains_key(&key) || score + 1000 < *best_score.get(&key).unwrap() {
            best_score.insert(key, score + 1000);
            best_previous.insert(key, vec![(pos, dir)]);
        } else if score + 1000 == *best_score.get(&key).unwrap() {
            best_previous.get_mut(&key).unwrap().push((pos, dir));
        }
        queue.push(State {
            pos,
            dir: dir.rotate(90),
            score: score + 1000,
        });

        let key = (pos, dir.rotate(-90));
        if !best_score.contains_key(&key) || score + 1000 < *best_score.get(&key).unwrap() {
            best_score.insert(key, score + 1000);
            best_previous.insert(key, vec![(pos, dir)]);
        } else if score + 1000 == *best_score.get(&key).unwrap() {
            best_previous.get_mut(&key).unwrap().push((pos, dir));
        }
        queue.push(State {
            pos,
            dir: dir.rotate(-90),
            score: score + 1000,
        });
    }

    Dir4::all()
        .iter()
        .flat_map(|d| get_all_routes(&(end.to_owned(), d.to_owned()), &best_previous))
        .flat_map(|route| route.iter().map(|(pos, _)| pos.clone()).collect_vec())
        .unique()
        .count()
}

fn get_all_routes(
    to: &(Pos2U, Dir4),
    best_previous: &HashMap<(Pos2U, Dir4), Vec<(Pos2U, Dir4)>>,
) -> Vec<Vec<(Pos2U, Dir4)>> {
    if let Some(prev) = best_previous.get(to) {
        return prev
            .iter()
            .flat_map(|prev| {
                get_all_routes(prev, best_previous)
                    .iter()
                    .map(|route| vec![route.clone(), vec![to.clone()]].concat())
                    .collect_vec()
            })
            .collect_vec();
    }
    vec![vec![to.clone()]]
}
