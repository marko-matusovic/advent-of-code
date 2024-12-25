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

        let mut robot = Pos2U(input.robot.0 * 2, input.robot.1);

        // print_p2(&walls, &boxes, &robot);

        for step in input.steps {
            // println!("Moving {}", step.to_string());
            if let Some(move_boxes) = robot_can_and_will_move(&walls, &mut boxes, &mut robot, &step)
            {
                robot = robot.add_unwrap(step.dir());

                for bx in move_boxes.iter() {
                    boxes.remove(bx);
                }

                for bx in move_boxes {
                    boxes.insert(bx.add_unwrap(step.dir()));
                }
                // println!("Moved")
                // } else {
                // println!("Halt")
            }

            // print_p2(&walls, &boxes, &robot);
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
            let next_w = next.add_unwrap(Dir4::W.dir());
            if boxes.contains(&next) {
                box_can_and_will_move(walls, boxes, &next, step)
            } else if boxes.contains(&next_w) {
                box_can_and_will_move(walls, boxes, &next_w, step)
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
            let next = next.add_unwrap(Dir4::E.dir());
            if walls.contains(&next) {
                return None;
            }
            if boxes.contains(&next) {
                return box_can_and_will_move(walls, boxes, &next, step)
                    .map(|bxs| vec![bxs, vec![bx.to_owned()]].concat());
            }
            Some(vec![bx.to_owned()])
        }
        Dir4::W => {
            if walls.contains(&next) {
                return None;
            }
            let next = next.add_unwrap(Dir4::W.dir());
            if boxes.contains(&next) {
                return box_can_and_will_move(walls, boxes, &next, step)
                    .map(|bxs| vec![bxs, vec![bx.to_owned()]].concat());
            }
            Some(vec![bx.to_owned()])
        }
        Dir4::S | Dir4::N => {
            let next_w = next.add_unwrap(Dir4::W.dir());
            let next_e = next.add_unwrap(Dir4::E.dir());
            if walls.contains(&next) || walls.contains(&next_e) {
                return None;
            }
            [next_w, next, next_e]
                .iter()
                .fold(Some(vec![bx.to_owned()]), |some_bxs, next_bx| {
                    if boxes.contains(next_bx) {
                        if let Some(bxs) = some_bxs {
                            box_can_and_will_move(walls, boxes, next_bx, step)
                                .map(|more_bxs| vec![bxs, more_bxs].concat())
                        } else {
                            None
                        }
                    } else {
                        some_bxs
                    }
                })
        }
    }
}

fn print_p2(walls: &HashSet<Pos2U>, boxes: &HashSet<Pos2U>, robot: &Pos2U) {
    let max = walls
        .iter()
        .max_by(|&a, &b| a.dominates(b.to_owned()).cmp(&false))
        .unwrap();

    let mut bx = false;
    for y in 0..=max.1 {
        for x in 0..=max.0 {
            if bx {
                print!("]");
                bx = false;
                continue;
            }

            let pos = Pos2U(x, y);
            if pos == *robot {
                print!("@");
            } else if walls.contains(&pos) {
                print!("#");
            } else if boxes.contains(&pos) {
                print!("[");
                bx = true;
            } else {
                print!(".");
            }
        }
        println!();
    }
}
