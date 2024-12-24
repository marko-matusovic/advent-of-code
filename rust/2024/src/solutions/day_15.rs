use std::collections::HashSet;

use itertools::Itertools;

use crate::libs::{dir_2d::Dir4, Pos2U};

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    robot: Pos2U,
    walls: HashSet<Pos2U>,
    boxes: HashSet<Pos2U>,
    steps: Vec<Dir4>,
}

fn parse_input(raw: &str) -> Input {
    let (grid, steps) = raw.split_once("\n\n").unwrap();

    let mut robot = None;
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();

    for (y, line) in grid.split("\n").into_iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    walls.insert(Pos2U(x, y));
                }
                '@' => {
                    robot = Some(Pos2U(x, y));
                }
                'O' => {
                    boxes.insert(Pos2U(x, y));
                }
                '.' => {}
                _ => panic!("Invalid character in input: {}", ch),
            }
        }
    }

    let steps = steps
        .chars()
        .filter_map(|ch| match ch {
            '^' => Some(Dir4::N),
            'v' => Some(Dir4::S),
            '<' => Some(Dir4::W),
            '>' => Some(Dir4::E),
            _ => None,
        })
        .collect_vec();

    Input {
        robot: robot.unwrap(),
        walls,
        boxes,
        steps,
    }
}

pub struct Day15;
impl Day for Day15 {
    fn day(&self) -> u8 {
        15
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let mut boxes = input.boxes.clone();
        let mut robot = input.robot;

        'steps: for step in input.steps {
            let mut cur = robot.clone();
            loop {
                cur = cur.add_unwrap(step.dir());
                if boxes.contains(&cur) {
                    continue;
                }
                if input.walls.contains(&cur) {
                    continue 'steps;
                }
                break;
            }
            robot = robot.add_unwrap(step.dir());
            boxes.insert(cur);
            boxes.remove(&robot);
        }

        let score: usize = boxes.iter().map(|b| b.0 + b.1 * 100).sum();

        println!("Answer is {}", score);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let walls = input
            .walls
            .iter()
            .flat_map(|&Pos2U(x, y)| [Pos2U(x * 2, y), Pos2U(x * 2 + 1, y)])
            .collect();

        let mut boxes = input
            .boxes
            .iter()
            .map(|&Pos2U(x, y)| Pos2U(x * 2, y))
            .collect();

        let mut robot = input.robot.clone();

        for step in input.steps {
            robot_can_and_will_move(&walls, &mut boxes, &mut robot, &step);
        }

        let score: usize = boxes.iter().map(|b| b.0 + b.1 * 100).sum();

        println!("Answer is {}", score);
    }
}

fn robot_can_and_will_move(
    walls: &HashSet<Pos2U>,
    boxes: &mut HashSet<Pos2U>,
    robot: &mut Pos2U,
    step: &Dir4,
) -> Option<Vec<Pos2U>> {
    let next = robot.add_unwrap(step.dir());
    if walls.contains(&next) {
        return None;
    }
    match step {
        Dir4::E => {
            if boxes.contains(&next) {
                box_can_and_will_move(walls, boxes, &next, step)
            } else {
                Some(Vec::new())
            }
        }
        Dir4::W => {
            let next = next.add_unwrap(Dir4::W.dir());
            if boxes.contains(&next) {
                box_can_and_will_move(walls, boxes, &next, step)
            } else {
                Some(Vec::new())
            }
        }
        Dir4::S | Dir4::N => {
            let second = next.add_unwrap(Dir4::W.dir());
            if boxes.contains(&next) {
                box_can_and_will_move(walls, boxes, &next, step)
            } else if boxes.contains(&second) {
                box_can_and_will_move(walls, boxes, &second, step)
            } else {
                Some(Vec::new())
            }
        }
    }
}

fn box_can_and_will_move(
    walls: &HashSet<Pos2U>,
    boxes: &mut HashSet<Pos2U>,
    bx: &Pos2U,
    step: &Dir4,
) -> Option<Vec<Pos2U>> {
    let next = bx.add_unwrap(step.dir());
    match step {
        Dir4::E => {
            let next = next.add_unwrap(step.dir());
            if walls.contains(&next) {
                return None;
            }
            if boxes.contains(&next) {
                return box_can_and_will_move(walls, boxes, &next, step);
            }
            Some(Vec::new())
        }
        Dir4::W => {
            if walls.contains(&next) {
                return None;
            }
            let next = next.add_unwrap(step.dir());
            if boxes.contains(&next) {
                return box_can_and_will_move(walls, boxes, &next, step);
            }
            Some(Vec::new())
        }
        Dir4::S | Dir4::N => {
            None
            // let second = next.add_unwrap(Dir4::W.dir());
            // (!boxes.contains(&next)
            //     || (boxes.contains(&next) && box_can_and_will_move(walls, boxes, &next, step)))
            //     && (!boxes.contains(&second)
            //         || (boxes.contains(&second)
            //             && box_can_and_will_move(walls, boxes, &second, step)))
        }
    }
}
