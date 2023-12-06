use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    races: Vec<(usize, usize)>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let races = lines
        .iter()
        .map(|l| {
            l.split(' ')
                .skip(1)
                .filter(|ch| !ch.is_empty())
                .map(|ch| ch.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect_vec()
        .chunks(2)
        .into_iter()
        .map(|slice| match slice {
            [t, d] => t.iter().map(|&t| t).zip(d.to_owned()).collect_vec(),
            _ => panic!(),
        })
        .find(|_| true)
        .unwrap();

    Input { lines, races }
}

pub struct Day06;
impl Day for Day06 {
    fn day(&self) -> u8 {
        06
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let prd: usize = input
            .races
            .iter()
            .map(|&(t, d)| {
                let tf: f64 = t as f64;
                let df: f64 = d as f64;

                let x1 = ((tf - (tf * tf - 4.0 * df).sqrt()) / 2.0).ceil() as usize;
                let x2 = ((tf + (tf * tf - 4.0 * df).sqrt()) / 2.0).floor() as usize;

                return x2 - x1 + 1;
            })
            .product();

        println!("Answer is {}", prd);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let vals: Vec<usize> = input
            .lines
            .iter()
            .map(|l| {
                l.split_once(": ")
                    .unwrap()
                    .1
                    .trim()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect();

        let time = vals.get(0).unwrap().to_owned() as f64;
        let dist = vals.get(1).unwrap().to_owned() as f64;

        let x1 = ((time - (time * time - 4.0 * dist).sqrt()) / 2.0).ceil() as usize;
        let x2 = ((time + (time * time - 4.0 * dist).sqrt()) / 2.0).floor() as usize;

        let res = x2 - x1 + 1;

        println!("Answer is {}", res);
    }
}
