use crate::libs::{dir::Dir4, pos_2i::Pos2I};

use super::day_trait::Day;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DigStruction {
    direction: Dir4,
    length: usize,
    color: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Input {
    lines: Vec<String>,
    digstructions: Vec<DigStruction>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();

    let digstructions = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();

            DigStruction {
                direction: match parts[0] {
                    "U" => Dir4::N,
                    "D" => Dir4::S,
                    "L" => Dir4::W,
                    "R" => Dir4::E,
                    _ => panic!(),
                },
                length: parts[1].parse().unwrap(),
                color: parts[2].to_owned(),
            }
        })
        .collect();

    Input {
        lines,
        digstructions,
    }
}

fn shoelace_area(points: &Vec<Pos2I>) -> f64 {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        sum1 += points[i].0 * points[j].1;
        sum2 += points[j].0 * points[i].1;
    }

    ((sum1 - sum2) as f64).abs() / 2.0
}

pub struct Day18;
impl Day for Day18 {
    fn day(&self) -> u8 {
        18
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let mut cur_pos = Pos2I(0, 0);
        let mut points: Vec<Pos2I> = Vec::new();
        points.push(cur_pos);

        for ins in input.digstructions.clone() {
            cur_pos = cur_pos + ins.direction.dir().scale(ins.length as isize);
            points.push(cur_pos);
        }

        let area = shoelace_area(&points)
            + 1.0
            + (input
                .digstructions
                .iter()
                .map(|ds| ds.length)
                .sum::<usize>() as f64)
                / 2.0;

        println!("Answer is {}", area);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let mut cur_pos = Pos2I(0, 0);
        let mut points: Vec<Pos2I> = Vec::new();
        points.push(cur_pos);

        let mut total_len: f64 = 0.0;

        for ins in input.digstructions.clone() {
            let len = usize::from_str_radix(&ins.color[2..=6], 16).unwrap();
            total_len += len as f64;
            let dir = match ins.color.chars().nth(7) {
                Some('0') => Dir4::E,
                Some('1') => Dir4::S,
                Some('2') => Dir4::W,
                Some('3') => Dir4::N,
                _ => panic!(),
            };

            cur_pos = cur_pos + dir.dir().scale(len as isize);
            points.push(cur_pos);
        }

        let area = shoelace_area(&points) + 1.0 + total_len / 2.0;

        println!("Answer is {}", area);
    }
}
