use super::day_trait::Day;
use cached::proc_macro::cached;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use pathfinding::num_traits::pow;
use rand::random;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter::once;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Input {
    totes: Vec<Tote>,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Tote {
    temperature: Temperature,
    weight: u32,
}

impl PartialOrd for Tote {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tote {
    fn cmp(&self, other: &Self) -> Ordering {
        self.temperature
            .qualifier()
            .cmp(&other.temperature.qualifier())
            .then(self.weight.cmp(&other.weight).reverse())
    }
}

impl std::fmt::Debug for Tote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ToString for Tote {
    fn to_string(&self) -> String {
        let prefix = match self.temperature {
            Temperature::AMBIENT => "A",
            Temperature::CHILD => "C",
            Temperature::FROZEN => "F",
        };
        format!("{}{:02}", prefix, self.weight)
    }
}

impl Tote {
    fn stacks_on(&self, other: &Tote) -> bool {
        self.temperature.qualifier() >= other.temperature.qualifier() && self.weight <= other.weight
    }
}

type Stack = Vec<Tote>;

type Frame = Vec<Stack>;

type Deployment = Vec<Frame>;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Temperature {
    AMBIENT,
    CHILD,
    FROZEN,
}

impl Temperature {
    fn qualifier(&self) -> i8 {
        match self {
            Temperature::AMBIENT => 1,
            Temperature::CHILD => 0,
            Temperature::FROZEN => -1,
        }
    }
}

fn parse_input(raw: &str) -> Input {
    Input {
        totes: raw
            .split(" ")
            .map(|s| match s.chars().nth(0).unwrap() {
                'A' => Tote {
                    temperature: Temperature::AMBIENT,
                    weight: s[1..].parse().unwrap(),
                },
                'C' => Tote {
                    temperature: Temperature::CHILD,
                    weight: s[1..].parse().unwrap(),
                },
                'F' => Tote {
                    temperature: Temperature::FROZEN,
                    weight: s[1..].parse().unwrap(),
                },
                _ => panic!("Unexpected character in input!"),
            })
            .collect(),
    }
}

pub struct DayPicnic;
impl Day for DayPicnic {
    fn day(&self) -> u8 {
        99
    }

    fn part_1(&self, raw: &str) {
        println!("Picnic Day part 1");
        let mut totes: VecDeque<Tote> = parse_input(raw).totes.into_iter().collect();

        for i in 0..=0 {
            println!("Input iteration {}", i);
            let solution = min_cost(totes.clone());
            println!("Answer is {}", cost_stacks(&solution));
            println!("Stacks: {:?}", solution);
            totes = semi_sort_v2(totes.into_iter().collect());
        }
    }

    fn part_2(&self, raw: &str) {
        println!("Picnic Day part 2");
        let totes: VecDeque<Tote> = parse_input(raw).totes.into_iter().collect();

        let solution = min_unbalance(totes, Vec::new());

        println!("Answer is {}", shipping_cost_deployment(&solution));
        println!("Frames: {:?}", solution);
    }
}

fn semi_sort(totes: VecDeque<Tote>) -> VecDeque<Tote> {
    let mut totes = totes.clone();
    let mut next_totes = VecDeque::new();
    let mut spare = totes.pop_front().unwrap();
    while let Some(next) = totes.pop_front() {
        if random::<usize>() % 5 == 0 {
            next_totes.push_back(next.clone());
            continue;
        }
        if !(next.stacks_on(&spare))
            && (spare.stacks_on(&next)
                || next.temperature.qualifier() < spare.temperature.qualifier()
                || next.weight > spare.weight)
        {
            next_totes.push_back(next.clone());
        } else {
            next_totes.push_back(spare.clone());
            spare = next;
        }
    }
    next_totes.push_back(spare);

    for chunk in totes.iter().chunks(2).into_iter() {
        let chunk: Vec<&Tote> = chunk.collect();
        if chunk.len() == 1 {
            next_totes.push_back(chunk[0].clone());
            break;
        }
        if !(chunk[1].stacks_on(chunk[0]))
            && (chunk[0].stacks_on(chunk[1])
                || chunk[1].temperature.qualifier() < chunk[0].temperature.qualifier()
                || chunk[1].weight > chunk[0].weight)
        {
            next_totes.push_back(chunk[1].clone());
            next_totes.push_back(chunk[0].clone());
        } else {
            next_totes.push_back(chunk[0].clone());
            next_totes.push_back(chunk[1].clone());
        }
    }
    next_totes
}

fn semi_sort_v2(totes: VecDeque<Tote>) -> VecDeque<Tote> {
    let mut totes = totes.clone();
    let mut sorted = VecDeque::new();
    while !totes.is_empty() {
        let mut group = Vec::new();
        while let Some(next) = totes.pop_front() {
            group.push(next);
            if random::<usize>() % 2 == 0 {
                break;
            }
        }

        group
            .iter()
            .sorted()
            .for_each(|t| sorted.push_back(t.clone()));
    }

    sorted
}

// PART 1
#[cached]
fn min_cost(totes: VecDeque<Tote>) -> Vec<Stack> {
    if totes.is_empty() {
        panic!("No totes left!");
    }
    if totes.len() == 1 {
        return vec![totes.into_iter().collect()];
    }

    let mut totes = totes.clone();
    let mut stack = vec![totes.pop_front().unwrap()];

    let mut solutions: Vec<Vec<Stack>> = vec![];
    solutions.push(
        vec![stack.clone()]
            .into_iter()
            .chain(min_cost(totes.clone()))
            .collect(),
    );

    for _ in 2..=4 {
        if let Some(next) = totes.pop_front() {
            if !next.stacks_on(stack.last().unwrap()) {
                break;
            }
            stack.push(next);
            if !totes.is_empty() {
                solutions.push(
                    vec![stack.clone()]
                        .into_iter()
                        .chain(min_cost(totes.clone()))
                        .collect(),
                );
            } else {
                solutions.push(vec![stack.clone()]);
                break;
            }
        }
    }

    solutions
        .iter()
        .min_by_key(|&s| cost_stacks(s))
        .unwrap()
        .clone()
}

fn cost_stack(count: usize) -> u32 {
    match count {
        0 => panic!("No totes in a stack!"),
        1 => 50,
        2 => 25,
        3 => 10,
        4 => 0,
        _ => panic!("Too many totes in a stack!"),
    }
}

fn cost_stacks(stacks: &Vec<Vec<Tote>>) -> u32 {
    stacks.iter().map(|s| cost_stack(s.len())).sum()
}

// PART 2
#[cached]
fn min_unbalance(totes: VecDeque<Tote>, frame: Frame) -> Deployment {
    if totes.is_empty() {
        return vec![frame];
    }
    let sorted_frame: Frame = sort_stacks(&frame);
    if !frame.eq(&sorted_frame) {
        return min_unbalance(totes, sorted_frame);
    }

    let mut totes = totes.clone();
    let tote = totes.pop_front().unwrap();
    let frame: Frame = sorted_frame.clone();
    let mut solutions: Vec<Deployment> = vec![];

    // println!("Start state | # totes: {:?} tote: [{:?}] frame: {:?}", &totes.len(), &tote, &frame);

    // add tote to existing stacks
    for stack in frame
        .iter()
        .filter(|stack| stack.len() < 4 && tote.stacks_on(stack.last().unwrap()))
    {
        let next_frame: Frame = frame
            .iter()
            .filter(|&s| !s.eq(stack))
            .cloned()
            .chain(once(
                stack.iter().cloned().chain(once(tote.clone())).collect(),
            ))
            .collect();
        // println!("Adding tote [{:?}] to stack {:?}", &tote, &stack);
        // println!("Original frame {:?}", &frame);
        // println!("Modified frame {:?}", &next_frame);
        solutions.push(min_unbalance(totes.clone(), next_frame));
    }
    // add tote to a new stack
    if frame.len() < 3 {
        let next_frame: Frame = frame
            .clone()
            .into_iter()
            .chain(once(vec![tote.clone()]))
            .collect();
        // println!("Adding tote [{:?}] to new stack", &tote);
        // println!("Original frame {:?}", &frame);
        // println!("Modified frame {:?}", &next_frame);
        solutions.push(min_unbalance(totes.clone(), next_frame.clone()));
    }
    // ship frame
    if frame.len() > 1 {
        // println!("Cannot add tote [{:?}]", &tote);
        // println!("Shipping frame {:?}", &frame);
        solutions.push(
            vec![frame.clone()]
                .iter()
                .chain(min_unbalance(totes.clone(), vec![vec![tote]]).iter())
                .map(|f| f.clone())
                .collect(),
        );
    }

    solutions
        .iter()
        .min_by_key(|&deployment| shipping_cost_deployment(deployment))
        .unwrap()
        .clone()
}

fn sort_stacks(stacks: &Frame) -> Frame {
    stacks
        .into_iter()
        .sorted_by_key(|&stack| stack.iter().map(|t| t.to_string()).join(";"))
        .map(|s| s.clone())
        .collect()
}

fn shipping_cost_frame(frame: &Frame) -> u32 {
    if frame.is_empty() {
        return 0;
    }
    if frame.len() > 3 {
        panic!("Too many stacks in a frame!");
    }

    if let MinMax(min, max) = frame
        .iter()
        .map(|s| s.iter().map(|t| t.weight).sum::<u32>())
        .chain(once(0))
        .take(3)
        .minmax()
    {
        return 1000 + pow(max - min, 2);
    }
    dbg!(frame);
    panic!("This frame is hella weird!");
}

fn shipping_cost_deployment(deployment: &Deployment) -> u32 {
    deployment.iter().map(shipping_cost_frame).sum()
}
