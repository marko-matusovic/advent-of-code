use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Add,
    usize,
};

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

pub struct Day16;
impl Day for Day16 {
    fn day(&self) -> u8 {
        16
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let score = shortest_path(&input.obstacles, &input.start, &Dir4::E, &input.end).unwrap();

        println!("Answer is {}", score);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let best_score =
            shortest_path(&input.obstacles, &input.start, &Dir4::E, &input.end).unwrap();
        let routes = all_routes(&input.obstacles, &input.start, &input.end, &best_score);

        let seats: HashSet<Pos2U> = routes
            .iter()
            .flat_map(|route| route)
            .map(|pos| pos.pos.to_owned())
            .collect();

        println!("Answer is {}", seats.len());
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

fn shortest_path(
    obstacles: &HashSet<Pos2U>,
    start: &Pos2U,
    dir: &Dir4,
    end: &Pos2U,
) -> Option<usize> {
    let mut visited: HashSet<(Pos2U, Dir4)> = HashSet::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    queue.push(State {
        pos: start.to_owned(),
        dir: dir.to_owned(),
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

fn all_routes(
    obstacles: &HashSet<Pos2U>,
    start: &Pos2U,
    end: &Pos2U,
    max_cost: &usize,
) -> Vec<Vec<State>> {
    let mut routes: Vec<Vec<State>> = Vec::new();
    let mut queue: Vec<Vec<State>> = Vec::new();

    // meh, calculating this for every pos is inefficient, we could do dijkstra once (but I am lazy)
    let mut shortest_from: HashMap<(Pos2U, Dir4), usize> = HashMap::new();

    queue.push(vec![State {
        pos: start.clone(),
        dir: Dir4::E,
        score: 0,
    }]);

    while let Some(route) = queue.pop() {
        // println!("queue: {}, routes: {}, shortest: {}", queue.len(), routes.len(), shortest_from.len());

        let last = route.last().unwrap();

        if obstacles.contains(&last.pos) {
            continue;
        }

        if last.pos == *end {
            routes.push(route.to_owned());
        }

        if route
            .iter()
            .find(|pos| pos.pos == last.pos && pos.dir == last.dir && pos.score != last.score)
            .is_some()
        {
            continue;
        }

        let shortest = shortest_from
            .entry((last.pos, last.dir))
            .or_insert_with(|| shortest_path(obstacles, &last.pos, &last.dir, end).unwrap());
        if *max_cost < last.score + *shortest {
            continue;
        }

        // forward
        let mut next = route.clone();
        next.push(State {
            pos: last.pos.add_unwrap(last.dir.dir()),
            dir: last.dir,
            score: last.score + 1,
        });
        queue.push(next);

        // turn right
        let mut next = route.clone();
        next.push(State {
            pos: last.pos.add_unwrap(last.dir.rotate(90).dir()),
            dir: last.dir.rotate(90),
            score: last.score + 1001,
        });
        queue.push(next);

        // turn left
        let mut next = route.clone();
        next.push(State {
            pos: last.pos.add_unwrap(last.dir.rotate(-90).dir()),
            dir: last.dir.rotate(-90),
            score: last.score + 1001,
        });
        queue.push(next);
    }

    routes
}
