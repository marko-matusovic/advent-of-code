use itertools::Itertools;
use pathfinding::num_traits::Pow;

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    registers: Registers,
    ops: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

fn parse_input(raw: &str) -> Input {
    let lines = raw.split("\n").collect_vec();
    let a = lines[0].split_once(": ").unwrap().1.parse().unwrap();
    let b = lines[1].split_once(": ").unwrap().1.parse().unwrap();
    let c = lines[2].split_once(": ").unwrap().1.parse().unwrap();
    let ops = lines[4]
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|o| o.parse().unwrap())
        .collect_vec();
    Input {
        registers: Registers { a, b, c },
        ops,
    }
}

pub struct Day17;
impl Day for Day17 {
    fn day(&self) -> u8 {
        17
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let Input { mut registers, ops }: Input = parse_input(raw);

        let mut pointer = 0;

        let mut outs = Vec::new();
        while let (Some(opcode), Some(operand)) = (ops.get(pointer), ops.get(pointer + 1)) {
            if let Some(out) = eval(&opcode, &operand, &mut pointer, &mut registers) {
                outs.push(out.to_string());
            }
        }

        println!("Answer is {}", outs.join(","));
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let _input: Input = parse_input(raw);

        println!("Answer is {}", 0);
    }
}

fn combo_operand(operand: &usize, registers: &Registers) -> usize {
    match operand {
        0 | 1 | 2 | 3 => operand.to_owned(),
        4 => registers.a.to_owned(),
        5 => registers.b.to_owned(),
        6 => registers.c.to_owned(),
        _ => panic!("Unknown combo operand {}", operand),
    }
}

fn eval(
    opcode: &usize,
    literal_operand: &usize,
    pointer: &mut usize,
    registers: &mut Registers,
) -> Option<usize> {
    *pointer += 2;
    match opcode {
        5 => Some(out(literal_operand, pointer, registers)),
        _ => {
            match opcode {
                0 => adv(literal_operand, pointer, registers),
                1 => bxl(literal_operand, pointer, registers),
                2 => bst(literal_operand, pointer, registers),
                3 => jnz(literal_operand, pointer, registers),
                4 => bxc(literal_operand, pointer, registers),
                6 => bdv(literal_operand, pointer, registers),
                7 => cdv(literal_operand, pointer, registers),
                _ => panic!("Unknown OP {}", opcode),
            };
            None
        }
    }
}

fn adv(literal_operand: &usize, _pointer: &mut usize, registers: &mut Registers) {
    registers.a /= 2.pow(combo_operand(literal_operand, registers)) as usize
}

fn bdv(literal_operand: &usize, _pointer: &mut usize, registers: &mut Registers) {
    registers.b /= 2.pow(combo_operand(literal_operand, registers)) as usize
}

fn cdv(literal_operand: &usize, _pointer: &mut usize, registers: &mut Registers) {
    registers.c /= 2.pow(combo_operand(literal_operand, registers)) as usize
}

fn bxl(literal_operand: &usize, _pointer: &mut usize, registers: &mut Registers) {
    registers.b ^= literal_operand
}

fn bst(literal_operand: &usize, _pointer: &mut usize, registers: &mut Registers) {
    registers.b = combo_operand(literal_operand, registers) % 8;
}

fn jnz(literal_operand: &usize, pointer: &mut usize, registers: &mut Registers) {
    if registers.a != 0 {
        *pointer = *literal_operand
    }
}

fn bxc(_literal_operand: &usize, _pointer: &mut usize, registers: &mut Registers) {
    registers.b = registers.b ^ registers.c;
}

fn out(literal_operand: &usize, _pointer: &mut usize, registers: &mut Registers) -> usize {
    combo_operand(literal_operand, registers) % 8
}

// 2,1,5,5,2,3,3,6,7 bad
