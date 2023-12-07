use std::{
    cmp::max,
    collections::{BinaryHeap, HashSet},
    ops::{Add, Sub},
};

use regex::Regex;

pub fn day() -> u8 {
    19
}

#[derive(Debug, Hash, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Material {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Material {
    fn apply(self, foo: &dyn Fn(usize, usize) -> usize, other: Self) -> Self {
        Material {
            ore: foo(self.ore, other.ore),
            clay: foo(self.clay, other.clay),
            obsidian: foo(self.obsidian, other.obsidian),
            geode: foo(self.geode, other.geode),
        }
    }
    fn scale(self, scale: usize) -> Self {
        Material {
            ore: self.ore * scale,
            clay: self.clay * scale,
            obsidian: self.obsidian * scale,
            geode: self.geode * scale,
        }
    }
    fn sub_limit(self, other: Self) -> Self {
        self.apply(&|a, b| if a > b { a - b } else { 0 }, other)
    }
    fn sat_with(self, robots: Self) -> usize {
        let time_needed = self.apply(
            &|a: usize, b: usize| {
                if a == 0 {
                    return 0;
                }
                if b == 0 {
                    return usize::MAX - 1;
                }
                (a as f64 / b as f64).ceil() as usize
            },
            robots,
        );
        return [
            time_needed.ore,
            time_needed.clay,
            time_needed.obsidian,
            time_needed.geode,
        ]
        .iter()
        .max()
        .unwrap()
        .to_owned();
    }
}
impl Add for Material {
    type Output = Material;
    fn add(self, other: Self) -> Self::Output {
        self.apply(&|a, b| a + b, other)
    }
}
impl Sub for Material {
    type Output = Material;
    fn sub(self, other: Self) -> Self::Output {
        self.apply(&|a, b| a - b, other)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct BlueprintRobot {
    cost: Material,
    prod: Material,
}

impl BlueprintRobot {
    fn create(c1: usize, c2: usize, c3: usize, p1: usize, p2: usize, p3: usize, p4: usize) -> Self {
        BlueprintRobot {
            cost: Material {
                ore: c1,
                clay: c2,
                obsidian: c3,
                geode: 0,
            },
            prod: Material {
                ore: p1,
                clay: p2,
                obsidian: p3,
                geode: p4,
            },
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Blueprint {
    id: usize,
    plans: Vec<BlueprintRobot>,
}

impl Blueprint {
    fn max_(&self, foo: &dyn Fn(&BlueprintRobot) -> usize) -> usize {
        self.plans.iter().map(foo).max().unwrap().to_owned()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, PartialOrd)]
struct State {
    time_left: usize,
    stock: Material,
    robots: Material,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.stock
            .geode
            .cmp(&other.stock.geode)
            .then_with(|| self.robots.geode.cmp(&other.robots.geode))
    }
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    blueprints: Vec<Blueprint>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());

    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    let blueprints = lines
        .iter()
        .map(|l| {
            let capture = re.captures(l).unwrap();
            let get = |i: usize| capture.get(i).unwrap().as_str().parse::<usize>().unwrap();
            Blueprint {
                id: get(1),
                plans: vec![
                    BlueprintRobot::create(get(2), 0, 0, 1, 0, 0, 0),
                    BlueprintRobot::create(get(3), 0, 0, 0, 1, 0, 0),
                    BlueprintRobot::create(get(4), get(5), 0, 0, 0, 1, 0),
                    BlueprintRobot::create(get(6), 0, get(7), 0, 0, 0, 1),
                ],
            }
        })
        .collect();

    Input { lines, blueprints }
}

fn solve(bp: &Blueprint, time: usize) -> usize {
    let mut explored: HashSet<State> = HashSet::new();
    let mut states: BinaryHeap<State> = BinaryHeap::new();
    states.push(State {
        time_left: time - 1,
        robots: Material {
            ore: 1,
            ..Material::default()
        },
        stock: Material {
            ore: 1,
            ..Material::default()
        },
    });

    let msel = [
        |m: Material| m.ore,
        |m: Material| m.clay,
        |m: Material| m.obsidian,
        |m: Material| m.geode,
    ];

    let mut best = 0;
    while let Some(state) = states.pop() {
        if explored.contains(&state) {
            continue;
        }
        explored.insert(state.to_owned());
        best = max(best, state.stock.geode);

        // prune early if infeasible
        if state.stock.geode
            + state.robots.geode * state.time_left
            + state.time_left * (state.time_left + 1) / 2
            < best
        {
            continue;
        }

        for bpr in &bp.plans {
            // If there are enough robots to make the most expensive part, don't bother making more of those type
            if msel
                .iter()
                .take(3)
                .any(|sel| sel(bpr.prod) == 1 && bp.max_(&|bpr| sel(bpr.cost)) <= sel(state.robots))
            {
                continue;
            }

            // add state
            let time_needed = 1 + bpr.cost.sub_limit(state.stock).sat_with(state.robots);

            if time_needed < state.time_left {
                states.push(State {
                    time_left: state.time_left - time_needed,
                    robots: state.robots + bpr.prod,
                    stock: state.stock + state.robots.scale(time_needed) - bpr.cost,
                })
            }
        }

        if 0 < state.robots.geode {
            states.push(State {
                time_left: 0,
                robots: state.robots,
                stock: state.stock + state.robots.scale(state.time_left),
            })
        }
    }

    return best;
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let sum: usize = input
        .blueprints
        .iter()
        .map(|bp| bp.id * solve(bp, 24))
        .sum();

    println!("Answer is {}", sum);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let product: usize = input
        .blueprints
        .iter()
        .take(3)
        .map(|bp| solve(bp, 32))
        .product();

    println!("Answer is {}", product);
}
