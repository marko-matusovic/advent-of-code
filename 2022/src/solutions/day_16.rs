use std::hash::Hash;

use cached::proc_macro::cached;

pub fn day() -> u8 {
    16
}

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub struct Input {
    valves: Vec<Valve>,
}

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
struct Valve {
    name: String,
    flow: usize,
    neighbors: Vec<String>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());

    let valves = lines
        .iter()
        .map(|l| {
            // Valve AA has flow rate=0; tunnels lead to valves AA, AA, AA
            let (_, l) = l.split_once("Valve ").unwrap();
            let (name, l) = l.split_once(" has flow rate=").unwrap();
            let (flow, mut neighs) = l.split_once("; ").unwrap();
            if neighs.starts_with("tunnels") {
                neighs = &neighs[23..];
            } else {
                neighs = &neighs[22..];
            }
            // (
            //     name.to_owned(),
            Valve {
                name: name.to_owned(),
                flow: flow.parse().unwrap(),
                neighbors: neighs.split(", ").map(|n| n.to_owned()).collect(),
            }
            // )
        })
        .collect();

    Input { valves }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    // let max_score = solve_part_1(input, &mut HashMap::new(), "AA", &Vec::new(), 30);
    let max_score = solve_part_1(&input.valves, "AA", &Vec::new(), 30);

    println!("Answer is {}", max_score);
}

// #[cached]
fn solve_part_1(valves: &Vec<Valve>, pos: &str, open: &Vec<String>, time_left: usize) -> usize {
    if time_left <= 1 {
        return 0;
    }

    let this_valve = valves.iter().find(|v| v.name.eq(pos)).unwrap();

    let mut scores = Vec::new();
    scores.push(0);
    // option 1: open current valve
    if !open.contains(&pos.to_owned()) {
        let mut new_open = open.clone();
        new_open.push(pos.to_owned());
        scores.push(
            this_valve.flow * (time_left - 1) + solve_part_1(valves, pos, &new_open, time_left - 1),
        )
    }

    for n in this_valve.neighbors.iter() {
        if !open.contains(n) {
            scores.push(solve_part_1(valves, n, &open, time_left - 1))
        }
    }

    scores.iter().max().unwrap().to_owned()
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    println!("Answer is {}", 0);
}
