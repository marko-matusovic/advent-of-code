use std::collections::HashMap;

use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    states: HashMap<String, bool>,
    gates: HashMap<String, Gate>,
}

#[derive(Debug, Clone)]
struct Gate(String, String, Op);

#[derive(Debug, Clone)]
enum Op {
    AND,
    XOR,
    OR,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Op::AND,
            "XOR" => Op::XOR,
            "OR" => Op::OR,
            _ => panic!("Invalid operation: {}", value),
        }
    }
}
impl Op {
    fn eval(&self, a: &bool, b: &bool) -> bool {
        match self {
            Op::AND => a & b,
            Op::XOR => a ^ b,
            Op::OR => a | b,
        }
    }
}

fn parse_input(raw: &str) -> Input {
    let (states, gates) = raw.split_once("\n\n").unwrap();
    let states = states
        .split("\n")
        .map(|state| state.split_once(": ").unwrap())
        .fold(HashMap::new(), |mut hm, state| {
            hm.insert(state.0.to_owned(), state.1 == "1");
            hm
        });
    let gates = gates
        .split("\n")
        .map(|gate| {
            let (inp, out) = gate.split_once(" -> ").unwrap();
            let (a, op, b) = inp.split(" ").collect_tuple().unwrap();
            (
                out.to_owned(),
                Gate(a.to_owned(), b.to_owned(), Op::from(op)),
            )
        })
        .fold(HashMap::new(), |mut hm, gate| {
            hm.insert(gate.0, gate.1);
            hm
        });
    Input { states, gates }
}

pub struct Day24;
impl Day for Day24 {
    fn day(&self) -> u8 {
        24
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let Input { mut states, gates } = parse_input(raw);

        let number: usize = gates
            .keys()
            .filter(|key| key.starts_with("z"))
            .map(|key| (key, eval(key, &gates, &mut states)))
            .fold(0, |num, (key, state)| match state {
                true => num | (1 << key.clone().split_off(1).parse::<usize>().unwrap()),
                false => num,
            });

        println!("Answer is {}", number);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let Input { mut states, gates } = parse_input(raw);
        
        println!("Answer is {}", 0);
    }
}

fn eval(gate: &str, gates: &HashMap<String, Gate>, states: &mut HashMap<String, bool>) -> bool {
    if let Some(&state) = states.get(gate) {
        return state;
    }
    let Gate(a, b, op) = gates.get(gate).unwrap();
    let state = op.eval(&eval(a, gates, states), &eval(b, gates, states));
    states.insert(gate.to_owned(), state);
    state
}
