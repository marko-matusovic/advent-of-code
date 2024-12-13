use std::{cmp::min, ops::Sub};

use crate::libs::{
    math::{divides, is_whole, solve_linear_system},
    Pos2U,
};
use itertools::Itertools;
use regex::Regex;

use super::day_trait::Day;

#[derive(Debug)]
struct Machine {
    button_a: Pos2U,
    button_b: Pos2U,
    prize: Pos2U,
}

fn parse_input(raw: &str) -> Vec<Machine> {
    let re =
        Regex::new("Button A: X\\+(\\d+), Y\\+(\\d+)\\nButton B: X\\+(\\d+), Y\\+(\\d+)\\nPrize: X=(\\d+), Y=(\\d+)").unwrap();

    raw.split("\n\n")
        .map(|machine| {
            let caps: Vec<usize> = re
                .captures(machine)
                .unwrap()
                .iter()
                .skip(1)
                .map(|d| d.unwrap().as_str().parse().unwrap())
                .collect_vec();

            Machine {
                button_a: Pos2U(caps[0], caps[1]),
                button_b: Pos2U(caps[2], caps[3]),
                prize: Pos2U(caps[4], caps[5]),
            }
        })
        .collect_vec()
}

pub struct Day13;
impl Day for Day13 {
    fn day(&self) -> u8 {
        13
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let machines = parse_input(raw);

        let cost: usize = machines
            .iter()
            .flat_map(|machine| find_min_cost_naive(machine))
            .sum();

        println!("Answer is {}", cost);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let machines = parse_input(raw)
            .iter()
            .map(
                |&Machine {
                     button_a,
                     button_b,
                     prize: Pos2U(x, y),
                 }| Machine {
                    button_a,
                    button_b,
                    prize: Pos2U(x + 10000000000000, y + 10000000000000),
                },
            )
            .collect_vec();

        let cost: usize = machines
            .iter()
            .flat_map(|machine| find_min_cost_math(machine))
            .sum();

        println!("Answer is {}", cost);
    }
}

fn find_min_cost_naive(machine: &Machine) -> Option<usize> {
    // find i, j such that:
    // x: i * m.a.0 + j * m.b.0 = m.r.0
    // y: i * m.a.1 + j * m.b.1 = m.r.1
    // min 3 * i + 1 * j

    let max_b = min(
        machine.prize.0 / machine.button_b.0,
        machine.prize.1 / machine.button_b.1,
    );
    let mut b = max_b;
    loop {
        if let Ok(Pos2U(x, y)) = machine.prize.sub(machine.button_b.scale(b)).try_into() {
            if divides(x, machine.button_a.0)
                && x as f64 / machine.button_a.0 as f64 == y as f64 / machine.button_a.1 as f64
            {
                let a = x / machine.button_a.0;
                // println!("{}x{}", a, b);
                break Some(3 * a + b);
            }
        }
        if b == 0 {
            break None;
        }
        b -= 1;
    }
}

fn find_min_cost_math(machine: &Machine) -> Option<usize> {
    let a = [
        [machine.button_a.0 as f64, machine.button_b.0 as f64],
        [machine.button_a.1 as f64, machine.button_b.1 as f64],
    ];
    let y = [machine.prize.0 as f64, machine.prize.1 as f64];
    let x = solve_linear_system(&a, &y).unwrap();
    if x[0] >= 0. && x[1] >= 0. && is_whole(x[0]) && is_whole(x[1]) {
        // println!("{}x{}", x[0], x[1]);
        return Some((x[0] * 3. + x[1]) as usize);
    }
    None
}

// 621028590204557 too high
