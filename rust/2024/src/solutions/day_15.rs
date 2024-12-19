use std::{collections::HashSet, ops::Add};

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
                cur = cur.add(step.dir()).try_into().unwrap();
                if boxes.contains(&cur) {
                    continue;
                }
                if input.walls.contains(&cur) {
                    continue 'steps;
                }
                break;
            }
            robot = robot.add(step.dir()).try_into().unwrap();
            boxes.insert(cur);
            boxes.remove(&robot);
        }

        let score: usize = boxes.iter().map(|b| b.0 + b.1 * 100).sum();

        println!("Answer is {}", score);
    }

    fn part_2(&self, _raw: &str) {
        println!("Day {} part 2", self.day());
        panic!("WIP");
        // let input: Input = parse_input(raw);

        // let walls = input
        //     .walls
        //     .iter()
        //     .flat_map(|&Pos2U(x, y)| [Pos2U(x * 2, y), Pos2U(x * 2 + 1, y)])
        //     .collect();

        // let mut boxes = input
        //     .boxes
        //     .iter()
        //     .map(|&Pos2U(x, y)| Pos2U(x * 2, y))
        //     .collect();

        // let mut robot = input.robot.clone();

        // // move the boxes around
        // for step in input.steps {
        //     let bx = Pos2U(0, 0);
        //     if let Some(boxes_to_move) = can_and_will_move(&walls, &boxes, &bx, &step) {}
        // }

        // let score: usize = boxes.iter().map(|b| b.0 + b.1 * 100).sum();

        // println!("Answer is {}", score);
    }
}

// fn can_and_will_move(
//     walls: &HashSet<Pos2U>,
//     boxes: &HashSet<Pos2U>,
//     bx: &Pos2U,
//     step: &Dir4,
// ) -> Option<Vec<Pos2U>> {
//     match step {
//         Dir4::E => {
//             let next = &Pos2U(bx.0 + 1, bx.1);
//             if walls.contains(next) {
//                 return can_and_will_move(walls, boxes, next, step)
//                     .map(|wm| vec![vec![next], wm].concat());
//             }
//             return Some(vec![bx.clone()]);
//         }
//         Dir4::W => {
//             let next = &Pos2U(bx.0 - 1, bx.1);
//             if walls.contains(next) {
//                 return can_and_will_move(walls, boxes, next, step)
//                     .map(|wm| vec![vec![next], wm].concat());
//             }
//             return Some(vec![bx.clone()]);
//         }
//         Dir4::S => todo!(),
//         Dir4::N => todo!(),
//     }
// }
