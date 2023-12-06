use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
    ops::Add,
};

use itertools::Itertools;

pub fn day() -> u8 {
    18
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos3D(isize, isize, isize);

impl Add for Pos3D {
    type Output = Pos3D;

    fn add(self, other: Self) -> Self::Output {
        Pos3D(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Pos3D {
    fn dominates(&self, other: Self) -> bool {
        return other.0 <= self.0 && other.1 <= self.1 && other.2 <= self.2;
    }
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    cubes: HashSet<Pos3D>,
}

const DIRS: [Pos3D; 6] = [
    Pos3D(1, 0, 0),
    Pos3D(0, 1, 0),
    Pos3D(0, 0, 1),
    Pos3D(-1, 0, 0),
    Pos3D(0, -1, 0),
    Pos3D(0, 0, -1),
];

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());

    let cubes = lines
        .iter()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse().unwrap())
                .collect_tuple::<(isize, isize, isize)>()
                .map(|(x, y, z)| Pos3D(x, y, z))
                .unwrap()
        })
        .collect();

    Input { lines, cubes }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let open_edges: usize = input
        .cubes
        .iter()
        .map(|&pos| {
            DIRS.iter()
                .map(|&diff| {
                    if input.cubes.contains(&(pos + diff)) {
                        return 0;
                    }
                    return 1;
                })
                .sum::<usize>()
        })
        .sum();

    println!("Answer is {}", open_edges);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    // find smallest encompassing cube

    let mut x_min = isize::MAX;
    let mut y_min = isize::MAX;
    let mut z_min = isize::MAX;
    let mut x_max = isize::MIN;
    let mut y_max = isize::MIN;
    let mut z_max = isize::MIN;

    for &Pos3D(x, y, z) in &input.cubes {
        x_min = min(x_min, x);
        y_min = min(y_min, y);
        z_min = min(z_min, z);
        x_max = max(x_max, x);
        y_max = max(y_max, y);
        z_max = max(z_max, z);
    }

    x_min -= 1;
    y_min -= 1;
    z_min -= 1;
    x_max += 1;
    y_max += 1;
    z_max += 1;

    let bound_min = Pos3D(x_min, y_min, z_min);
    let bound_max = Pos3D(x_max, y_max, z_max);

    // fill the cube with water from the outside

    let lava = input.cubes.to_owned();
    let mut water: HashSet<Pos3D> = HashSet::new();
    let mut exploring: VecDeque<Pos3D> = VecDeque::new();
    exploring.push_back(Pos3D(x_min, y_min, z_min));
    exploring.push_back(Pos3D(x_min, y_min, z_max));
    exploring.push_back(Pos3D(x_min, y_max, z_min));
    exploring.push_back(Pos3D(x_min, y_max, z_max));
    exploring.push_back(Pos3D(x_max, y_min, z_min));
    exploring.push_back(Pos3D(x_max, y_min, z_max));
    exploring.push_back(Pos3D(x_max, y_max, z_min));
    exploring.push_back(Pos3D(x_max, y_max, z_max));

    while let Some(pos) = exploring.pop_front() {
        if lava.contains(&pos)
            || water.contains(&pos)
            || !pos.dominates(bound_min)
            || !bound_max.dominates(pos)
        {
            continue;
        }
        water.insert(pos);
        DIRS.iter()
            .for_each(|&diff| exploring.push_back(pos + diff));
    }

    // check water-lava faces
    let edges: usize = lava
        .iter()
        .map(|&pos| {
            DIRS.iter()
                .map(|&diff| {
                    if water.contains(&(pos + diff)) {
                        return 1;
                    }
                    return 0;
                })
                .sum::<usize>()
        })
        .sum();

    println!("Answer is {}", edges);
}

// 2017 too low
