use std::collections::HashMap;

use itertools::Itertools;

pub fn day() -> u8 {
    11
}

#[derive(Debug)]
pub struct Input {
    monkeys: HashMap<usize, Monkey>,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Monkey {
    id: usize,
    items: Vec<i128>,
    operation: Operation,
    test_divisible_by: i128,
    forward_on_success: usize,
    forward_on_fail: usize,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
enum Operation {
    ADD(i128),
    MULT(i128),
    SQUARE,
}

fn exec(old: &i128, op: &Operation) -> i128 {
    match op {
        Operation::ADD(n) => old + n,
        Operation::MULT(n) => old * n,
        Operation::SQUARE => old * old,
    }
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} monkeys", day(), lines.len());
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();
    lines.iter().for_each(|m| {
        let ls: Vec<&str> = m.split("\n").collect();
        let id = ls[0][7..ls[0].len() - 1].parse().unwrap();
        monkeys.insert(
            id,
            Monkey {
                id: id,
                items: ls[1]
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split(", ")
                    .map(|d| d.parse().unwrap())
                    .collect(),
                operation: match ls[2].chars().nth(23).unwrap() {
                    '+' => Operation::ADD(ls[2].split_at(25).1.parse().unwrap()),
                    '*' => {
                        if ls[2].ends_with("old") {
                            Operation::SQUARE
                        } else {
                            Operation::MULT(ls[2].split_at(25).1.parse().unwrap())
                        }
                    }
                    other => panic!("other operation: {}", other),
                },
                test_divisible_by: ls[3].split_at(21).1.parse().unwrap(),
                forward_on_success: ls[4].split_at(29).1.parse().unwrap(),
                forward_on_fail: ls[5].split_at(30).1.parse().unwrap(),
            },
        );
    });

    Input { monkeys }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let mut monkeys = input.monkeys.clone();
    let order: Vec<usize> = monkeys.keys().map(|k| k.to_owned()).sorted().collect_vec();

    let mut monkey_handling: HashMap<usize, usize> = HashMap::new();
    for &id in &order {
        monkey_handling.insert(id, 0);
    }

    for _ in 0..20 {
        for &id in &order {
            let monkey = monkeys.get(&id).unwrap().clone();
            monkeys.get_mut(&id).unwrap().items.clear();
            for item in &monkey.items {
                monkey_handling.entry(id).and_modify(|v| *v += 1);
                let new: i128 = exec(&item, &monkey.operation.clone()) / 3;
                if new % monkey.test_divisible_by == 0 {
                    monkeys
                        .get_mut(&monkey.forward_on_success)
                        .unwrap()
                        .items
                        .push(new);
                } else {
                    monkeys
                        .get_mut(&monkey.forward_on_fail)
                        .unwrap()
                        .items
                        .push(new);
                }
            }
        }
    }

    let times: Vec<usize> = monkey_handling
        .values()
        .sorted()
        .rev()
        .map(|v| v.to_owned())
        .collect();

    println!(
        "Answer is {}",
        times.get(0).unwrap() * times.get(1).unwrap()
    );
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let mut monkeys = input.monkeys.clone();
    let order: Vec<usize> = monkeys.keys().map(|k| k.to_owned()).sorted().collect_vec();

    let mut monkey_handling: HashMap<usize, usize> = HashMap::new();
    for &id in &order {
        monkey_handling.insert(id, 0);
    }

    let common_factor: i128 = monkeys
        .values()
        .map(|m| m.test_divisible_by.clone())
        .product();

    for _ in 0..10000 {
        for &id in &order {
            let monkey = monkeys.get(&id).unwrap().clone();
            monkeys.get_mut(&id).unwrap().items.clear();
            for item in &monkey.items {
                monkey_handling.entry(id).and_modify(|v| *v += 1);
                let new: i128 = exec(&item, &monkey.operation.clone()) % common_factor;
                if new % monkey.test_divisible_by == 0 {
                    monkeys
                        .get_mut(&monkey.forward_on_success)
                        .unwrap()
                        .items
                        .push(new);
                } else {
                    monkeys
                        .get_mut(&monkey.forward_on_fail)
                        .unwrap()
                        .items
                        .push(new);
                }
            }
        }
    }

    let times: Vec<usize> = monkey_handling
        .values()
        .sorted()
        .rev()
        .map(|v| v.to_owned())
        .collect();

    println!(
        "Answer is {}",
        times.get(0).unwrap() * times.get(1).unwrap()
    );

    // 135605001 too low
}
