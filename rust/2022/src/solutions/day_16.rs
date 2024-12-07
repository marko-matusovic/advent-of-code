use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use itertools::Itertools;

pub fn day() -> u8 {
    16
}

#[derive(Clone)]
pub struct Input {
    valves: HashMap<String, Valve>,
    distances: HashMap<String, HashMap<String, usize>>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Valve {
    id: isize,
    flow: usize,
    neighbors: HashSet<String>,
}

type Cache = HashMap<(String, usize, u32), usize>;

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());

    let mut valves: HashMap<String, Valve> = HashMap::new();

    for l in lines {
        // Valve AA has flow rate=0; tunnels lead to valves AA, AA, AA
        let (_, l) = l.split_once("Valve ").unwrap();
        let (name, l) = l.split_once(" has flow rate=").unwrap();
        let (flow, mut neighs) = l.split_once("; ").unwrap();
        if neighs.starts_with("tunnels") {
            neighs = &neighs[23..];
        } else {
            neighs = &neighs[22..];
        }
        valves.insert(
            name.to_owned(),
            Valve {
                id: -1,
                flow: flow.parse().unwrap(),
                neighbors: neighs.split(", ").map(|n| n.to_owned()).collect(),
            },
        );
    }

    let mut distances = HashMap::new();

    for (valve_id, valve) in &valves {
        if valve_id != "AA" && valve.flow == 0 {
            continue;
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((valve_id.clone(), 0));
        visited.insert(valve_id.clone());

        while let Some((current_id, distance)) = queue.pop_front() {
            if let Some(neighbors) = valves.get(&current_id).map(|v| &v.neighbors) {
                for neighbor_id in neighbors {
                    if !visited.contains(neighbor_id) {
                        queue.push_back((neighbor_id.clone(), distance + 1));
                        visited.insert(neighbor_id.clone());
                        distances
                            .entry(valve_id.clone())
                            .or_insert_with(HashMap::new)
                            .insert(neighbor_id.clone(), distance + 1);
                    }
                }
            }
        }
    }

    let zero_flow_valves: HashSet<&String> = valves
        .iter()
        .filter(|&(k, v)| v.flow == 0 && k != "AA")
        .map(|(k, _)| k)
        .collect();

    distances.iter_mut().for_each(|(_, v)| {
        zero_flow_valves.iter().for_each(|&zfv| {
            v.remove(zfv);
        })
    });

    let mut ids = 0;
    for v in distances.keys().sorted() {
        valves.get_mut(v).unwrap().id = ids;
        ids += 1;
    }

    Input { valves, distances }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let mut cache = Cache::new();
    let max = find_max_flow(input, &mut cache, "AA", 30, 0);

    println!("Answer is {}", max);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());
    
    let mut cache = Cache::new();
    let max_id = input.valves.values().map(|v| v.id).max().unwrap();
    let search_space = ((1 << (max_id + 1)) - 1) / 2;

    let mut max_flow = 0;

    for i in 0..search_space {
        let me = find_max_flow(input, &mut cache, "AA", 26, i);
        let el = find_max_flow(input, &mut cache, "AA", 26, search_space ^ i);
        if me + el > max_flow {
            max_flow = me + el;
        }
    }

    println!("Answer is {}", max_flow);
}

fn find_max_flow(
    input: &Input,
    cache: &mut Cache,
    cur_valve: &str,
    time_remaining: usize,
    visited: u32,
) -> usize {
    let key = (cur_valve.to_owned(), time_remaining, visited);
    if let Some(val) = cache.get(&key) {
        return *val;
    }
    
    let mut max_flow = 0;
    for (neigh, dist) in input.distances.get(cur_valve).unwrap() {
        let mask = 1 << input.valves.get(neigh).unwrap().id;
        if visited & mask > 0 {
            continue;
        }
        if time_remaining <= dist + 1 {
            continue;
        }
        let new_time = time_remaining - dist - 1;
        let gained_flow = input.valves.get(neigh).unwrap().flow * new_time;
        let flow = find_max_flow(input, cache, neigh, new_time, visited | mask) + gained_flow;
        if flow > max_flow {
            max_flow = flow;
        }
    }

    cache.insert(key, max_flow);
    return max_flow;
}
