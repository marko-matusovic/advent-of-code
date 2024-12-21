use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    ops::Add,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    pos: Pos2U,
    cheating: usize,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .cmp(&other.cost)
            .reverse()
            .then(self.cheating.cmp(&other.cheating))
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

        let min_cost = least_cost(&input.obstacles, &input.start, &input.end, 0).unwrap();

        let count = all_routes(
            &input.obstacles,
            &input.start,
            &input.end,
            1,
            min_cost - 100,
        )
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

fn least_cost(
    obstacles: &HashSet<Pos2U>,
    start: &Pos2U,
    end: &Pos2U,
    cheating: usize,
) -> Option<usize> {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashSet<Pos2U> = HashSet::new();
    queue.push(State {
        pos: start.clone(),
        cheating,
        cost: 0,
    });

    while let Some(State {
        pos,
        cheating,
        cost,
    }) = queue.pop()
    {
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
                        cheating,
                        cost: cost + 1,
                    });
                } else if cheating > 0 {
                    queue.push(State {
                        pos: next,
                        cheating: cheating - 1,
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
    max_cost: usize,
) -> Vec<Vec<Pos2U>> {
    // all_routes_rec(obstacles, &HashSet::new(), start, end, cheating, max_cost).unwrap_or_else(|| Vec::new())
    all_routes_iter(obstacles, start, end, cheating, max_cost)
}

// fn all_routes_rec(
//     obstacles: &HashSet<Pos2U>,
//     visited: &HashSet<Pos2U>,
//     current: &Pos2U,
//     end: &Pos2U,
//     cheating: usize,
//     max_cost: usize,
// ) -> Option<Vec<Vec<Pos2U>>> {
//     if current == end {
//         return Some(vec![vec![end.clone()]]);
//     }

//     print!("{}, ", max_cost);
//     if max_cost <= 0 {
//         return None;
//     }

//     let mut visited = visited.clone();
//     if !visited.insert(current.clone()) {
//         return None;
//     }

//     Dir4::all()
//         .iter()
//         .map(|dir| {
//             if let Ok(next) = TryInto::<Pos2U>::try_into(current.add(dir.dir())) {
//                 let routes = match (obstacles.contains(&next), cheating > 0) {
//                     (false, _) => {
//                         all_routes_rec(obstacles, &visited, &next, end, cheating, max_cost - 1)
//                     }
//                     (true, true) => {
//                         all_routes_rec(obstacles, &visited, &next, end, cheating - 1, max_cost - 1)
//                     }
//                     (_, _) => None,
//                 };
//                 return routes.map(|routes| {
//                     routes
//                         .iter()
//                         .map(|route| vec![vec![current.clone()], route.clone()].concat())
//                         .collect_vec()
//                 });
//             }
//             None
//         })
//         .fold(None, |a, b| match (a, b) {
//             (Some(mut x), Some(y)) => {
//                 x.extend(y);
//                 Some(x)
//             }
//             (a, None) => a,
//             (None, b) => b,
//         })
// }

fn all_routes_iter(
    obstacles: &HashSet<Pos2U>,
    start: &Pos2U,
    end: &Pos2U,
    cheating: usize,
    max_cost: usize,
) -> Vec<Vec<Pos2U>> {
    let mut least_cost_from: HashMap<(Pos2U, usize), usize> = HashMap::new();
    let mut queue: VecDeque<(Vec<Pos2U>, Pos2U, usize)> = VecDeque::new();
    queue.push_back((vec![], start.clone(), cheating));

    let mut all_routes = Vec::new();
    // print!("\r{}", all_routes.len());
    while let Some((route, pos, cheating)) = queue.pop_front() {
        if route.contains(&pos) {
            continue;
        }

        if max_cost
            < route.len()
                + *least_cost_from.entry((pos, cheating)).or_insert_with(|| {
                    least_cost(obstacles, &pos, end, cheating).unwrap_or(usize::MAX)
                })
        {
            continue;
        }

        let route = vec![route, vec![pos]].concat();

        if pos == *end {
            all_routes.push(route);
            println!("queue:{}, routes:{}, lcf:{}", queue.len(), all_routes.len(), least_cost_from.len());
            continue;
        }

        for dir in Dir4::all() {
            if let Ok(next) = TryInto::<Pos2U>::try_into(pos.add(dir.dir())) {
                if !obstacles.contains(&next) {
                    queue.push_front((route.clone(), next, cheating));
                } else if cheating >= 1 {
                    queue.push_front((route.clone(), next, cheating - 1));
                }
            }
        }
    }

    all_routes
}
