use super::day_trait::Day;
use cached::proc_macro::cached;
use itertools::Itertools;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Input {
    machines: Vec<Machine>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Machine {
    lights: u16,
    buttons: Vec<Vec<u16>>,
    joltage: Vec<u16>,
}

fn parse_input(raw: &str) -> Input {
    Input {
        machines: raw
            .lines()
            .map(|line| {
                let (goal_str, rest) = line[1..].split_once("] (").unwrap();
                let (buttons_str, rest) = rest.split_once(") {").unwrap();
                let joltage_str = rest.trim_end_matches('}');

                Machine {
                    lights: goal_str.chars().enumerate().fold(0, |inter, (i, ch)| {
                        inter
                            + match ch {
                                '.' => 0,
                                '#' => 1 << i,
                                _ => panic!("Invalid character in goal string"),
                            }
                    }),
                    buttons: buttons_str
                        .split(") (")
                        .map(|button_str| {
                            button_str
                                .split(',')
                                .map(|num| num.parse().unwrap())
                                .collect()
                        })
                        .collect(),
                    joltage: joltage_str
                        .split(',')
                        .map(|num| num.parse().unwrap())
                        .collect(),
                }
            })
            .collect(),
    }
}

pub struct Day10;
impl Day for Day10 {
    fn day(&self) -> u8 {
        10
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let machines = parse_input(raw).machines;

        let score: usize = machines
            .iter()
            .map(|machine| {
                least_clicks_lights(
                    machine.lights.clone(),
                    machine
                        .buttons
                        .iter()
                        .map(|button| {
                            button
                                .iter()
                                .map(|num| 1 << num)
                                .reduce(|a, b| a | b)
                                .unwrap()
                        })
                        .sorted()
                        .collect(),
                )
            })
            .sum();

        println!("Answer is {}", score);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let machines = parse_input(raw).machines;

        let score: usize = machines
            .iter()
            .map(|machine| least_clicks_joltage(machine.joltage.clone(), machine.buttons.clone()))
            .sum();

        println!("Answer is {}", score);
    }
}

#[cached]
fn least_clicks_lights(lights: u16, buttons: Vec<u16>) -> usize {
    if lights == 0 {
        return 0;
    }
    buttons
        .iter()
        .map(|button| {
            1 + least_clicks_lights(
                lights ^ button,
                buttons
                    .iter()
                    .filter(|b2| *b2 != button)
                    .sorted()
                    .cloned()
                    .collect(),
            )
        })
        .min()
        .unwrap_or(999_999_999)
}

#[cached]
fn least_clicks_joltage(joltage: Vec<u16>, buttons: Vec<Vec<u16>>) -> usize {
    println!("{:?}", joltage);
    if joltage.iter().all(|&num| num == 0) {
        return 0;
    }
    buttons
        .iter()
        .filter_map(|button| {
            let mut new_joltage = joltage.clone();
            let min_num = button
                .iter()
                .map(|pos| joltage[*pos as usize])
                .min()
                .unwrap();
            if min_num == 0 {
                return None;
            }
            for &pos in button {
                new_joltage[pos as usize] -= min_num;
            }
            println!(
                "{:?} - {} * {:?} = {:?}",
                joltage, min_num, button, new_joltage
            );
            return Some(min_num as usize + least_clicks_joltage(new_joltage, buttons.clone()));
        })
        .min()
        .unwrap_or(999_999_999)
}
