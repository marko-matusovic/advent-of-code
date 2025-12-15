use super::day_trait::Day;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
    shapes: HashMap<usize, Vec<Vec<bool>>>,
    trees: Vec<((usize, usize), HashMap<usize, usize>)>,
}

fn parse_input(raw: &str) -> Input {
    Input {
        shapes: raw
            .split("\n\n")
            .map_while(|raw_shape| {
                if let Some((i, raw_shape)) = raw_shape.split_once(":\n") {
                    Some((
                        i.parse().unwrap(),
                        raw_shape
                            .lines()
                            .map(|line| line.chars().map(|c| c == '#').collect())
                            .collect(),
                    ))
                } else {
                    None
                }
            })
            .collect(),
        trees: raw
            .split("\n\n")
            .tail(1)
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let (dims, counts) = line.split_once(": ").unwrap();
                let (x, y) = dims
                    .split("x")
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                (
                    (x, y),
                    counts
                        .split(' ')
                        .map(|n| n.parse().unwrap())
                        .enumerate()
                        .collect(),
                )
            })
            .collect(),
    }
}

pub struct Day12;
impl Day for Day12 {
    fn day(&self) -> u8 {
        12
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let Input { shapes, trees } = parse_input(raw);

        let shape_size: HashMap<usize, usize> = shapes
            .iter()
            .map(|(i, shape)| {
                (
                    i.clone(),
                    shape
                        .iter()
                        .map(|row| row.iter().filter(|x| **x).count())
                        .sum(),
                )
            })
            .collect();

        let fits_by_size = trees
            .iter()
            .filter(|((x, y), counts)| {
                counts.iter().map(|(i, c)| c * shape_size[i]).sum::<usize>() <= (x * y)
            })
            .count();

        println!("Answer is {}", fits_by_size);
    }

    fn part_2(&self, _raw: &str) {}
}
