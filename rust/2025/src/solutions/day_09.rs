use super::day_trait::Day;
use crate::libs::Pos2U;
use pathfinding::num_traits::abs;

#[derive(Debug)]
pub struct Input {
    tiles: Vec<Pos2U>,
}

fn parse_input(raw: &str) -> Input {
    Input {
        tiles: raw
            .lines()
            .map(|l| {
                l.split(",")
                    .map(|num| num.parse().unwrap())
                    .collect::<Pos2U>()
            })
            .collect(),
    }
}

pub struct Day09;
impl Day for Day09 {
    fn day(&self) -> u8 {
        09
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let tiles = parse_input(raw).tiles;

        let size = tiles
            .iter()
            .enumerate()
            .map(|(i, t1)| {
                tiles
                    .iter()
                    .skip(i + 1)
                    .map(|t2| {
                        (1 + abs(t1.0 as isize - t2.0 as isize) as usize)
                            * (1 + abs(t1.1 as isize - t2.1 as isize) as usize)
                    })
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap();

        println!("Answer is {}", size);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let _input: Input = parse_input(raw);

        println!("Answer is {}", 0);
    }
}
