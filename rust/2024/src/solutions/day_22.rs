use std::{
    collections::{HashMap, HashSet, VecDeque},
    usize,
};

use super::day_trait::Day;

fn parse_input(raw: &str) -> Vec<usize> {
    raw.split("\n").map(|s| s.parse().unwrap()).collect()
}

type Seq = (i8, i8, i8, i8);

pub struct Day22;
impl Day for Day22 {
    fn day(&self) -> u8 {
        22
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let monkeys = parse_input(raw);

        let sum: usize = monkeys
            .iter()
            .map(|&n| (0..2000).fold(n, |n, _| next_random(n)))
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let monkeys = parse_input(raw);

        let mut seq_to_cost: HashMap<Seq, isize> = HashMap::new();
        for monkey in monkeys {
            let mut monkey = monkey;
            let mut last_price = (monkey % 10) as i8;
            let mut seen: HashSet<Seq> = HashSet::new();
            let mut deltas = VecDeque::new();

            for i in 0..2000 {
                monkey = next_random(monkey);
                let price = (monkey % 10) as i8;
                deltas.push_front(price - last_price);
                last_price = price;

                if 3 <= i {
                    let seq = (deltas[3], deltas[2], deltas[1], deltas[0]);
                    deltas.pop_back();
                    if seen.insert(seq) {
                        *seq_to_cost.entry(seq).or_insert(0) += price as isize;
                    }
                }
            }
        }

        println!("Answer is {}", seq_to_cost.values().max().unwrap());
    }
}

fn next_random(n: usize) -> usize {
    let n = (n ^ (n * 64)) % 16777216;
    let n = (n ^ (n / 32)) % 16777216;
    let n = (n ^ (n * 2048)) % 16777216;
    n
}
