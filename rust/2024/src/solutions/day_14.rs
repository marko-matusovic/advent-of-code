use std::{collections::HashSet, ops::Add};

use itertools::Itertools;

use crate::libs::Pos2I;

use super::day_trait::Day;

#[derive(Debug)]
pub struct Robot {
    pos: Pos2I,
    vel: Pos2I,
}

fn parse_input(raw: &str) -> Vec<Robot> {
    raw.split("\n")
        .map(|s| {
            let (p, v) = s.split_once(" ").unwrap();
            let p: Vec<isize> = p
                .split_at(2)
                .1
                .split(",")
                .into_iter()
                .map(|n| n.parse().unwrap())
                .collect_vec();
            let v: Vec<isize> = v
                .split_at(2)
                .1
                .split(",")
                .into_iter()
                .map(|n| n.parse().unwrap())
                .collect_vec();
            Robot {
                pos: Pos2I(p[0], p[1]),
                vel: Pos2I(v[0], v[1]),
            }
        })
        .collect_vec()
}

pub struct Day14;
impl Day for Day14 {
    fn day(&self) -> u8 {
        14
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let dim = Pos2I(101, 103);
        let robots = parse_input(raw);

        let final_destinations = robots
            .iter()
            .map(|r| r.pos.add(r.vel.scale(100)).wrap(dim))
            .collect_vec();

        let mut counts = [0; 4];
        for robot in final_destinations {
            if robot.0 == 50 || robot.1 == 51 {
                continue;
            }

            if Pos2I(49, 50).dominates(robot) {
                counts[0] += 1;
            } else if Pos2I(100, 50).dominates(robot) {
                counts[1] += 1;
            } else if Pos2I(49, 102).dominates(robot) {
                counts[2] += 1;
            } else {
                counts[3] += 1;
            }
        }

        println!("Answer is {}", counts.iter().product::<isize>());
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let mut robots = parse_input(raw);
        let dim = Pos2I(101, 103);

        let mut seconds = 0;
        'all: loop {
            let positions: HashSet<Pos2I> = robots.iter().map(|r| r.pos).collect();
            for x in 0..101 {
                'coord: for y in 0..103 {
                    for i in 0..20 {
                        if !positions.contains(&Pos2I(x + i, y)) {
                            continue 'coord;
                        }
                    }
                    print(&positions);
                    break 'all;
                }
            }

            seconds += 1;
            robots = robots
                .iter()
                .map(|&Robot { pos, vel }| Robot {
                    pos: pos.add(vel).wrap(dim),
                    vel,
                })
                .collect_vec();
        }

        println!("Answer is {}", seconds);
    }
}

fn print(positions: &HashSet<Pos2I>) {
    for y in 0..103 {
        for x in 0..101 {
            if positions.contains(&Pos2I(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
