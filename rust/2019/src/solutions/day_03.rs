use std::{collections::{HashSet, HashMap}, hash::Hash};

pub fn day() -> u8 {
    3
}

#[derive(Debug)]
pub struct Input {
    wire_1: Vec<String>,
    wire_2: Vec<String>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<&str> = raw.split("\n").collect();
    println!("Day {} parsing {} lines", day(), lines.len());
    let paths: Vec<Vec<String>> = lines
        .iter()
        .map(|&l| l.split(",").map(|s| s.to_owned()).collect())
        .collect();
    Input {
        wire_1: paths[0].clone(),
        wire_2: paths[1].clone(),
    }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let points_1 = instruction_to_points(&input.wire_1);
    let points_2 = instruction_to_points(&input.wire_2);

    let min_distance = points_1
        .iter()
        .filter(|&p| points_2.contains(p))
        .map(|p| p.0.abs() + p.1.abs())
        .min()
        .unwrap();

    println!("Answer is {}", min_distance);
}

fn instruction_to_points(instructions: &Vec<String>) -> HashSet<(i32, i32)> {
    let mut points: HashSet<(i32, i32)> = HashSet::new();
    let mut current = (0, 0);
    for instruction in instructions {
        let (dir, amount) = instruction.split_at(1);
        let delta_dir = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            other => panic!("Unexpected direction: {}", &other),
        };
        for _ in 0..(amount.parse().unwrap()) {
            current.0 += delta_dir.0;
            current.1 += delta_dir.1;
            points.insert(current.clone());
        }
    }
    return points;
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());
    
    let (points_1, distances_1) = instruction_to_points_and_distance(&input.wire_1);
    let (points_2, distances_2) = instruction_to_points_and_distance(&input.wire_2);

    let min_distance = points_1
        .iter()
        .filter(|&p| points_2.contains(p))
        .map(|p| distances_1[p] + distances_2[p])
        .min()
        .unwrap();

    println!("Answer is {}", min_distance);
}

fn instruction_to_points_and_distance(instructions: &Vec<String>) -> (HashSet<(i32, i32)>, HashMap<(i32,i32), i32>) {
    let mut points: HashSet<(i32, i32)> = HashSet::new();
    let mut distances: HashMap<(i32, i32), i32> = HashMap::new();
    let mut current = (0, 0);
    let mut distance = 0;
    for instruction in instructions {
        let (dir, amount) = instruction.split_at(1);
        let delta_dir = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            other => panic!("Unexpected direction: {}", &other),
        };
        for _ in 0..(amount.parse().unwrap()) {
            distance += 1;
            current.0 += delta_dir.0;
            current.1 += delta_dir.1;
            points.insert(current.clone());
            distances.entry(current.clone()).or_insert(distance);
        }
    }
    return (points, distances);
}