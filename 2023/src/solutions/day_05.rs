use std::cmp::min;

use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug)]
pub struct CompMap {
    from: usize,
    to: usize,
    len: usize,
}

#[derive(Debug)]
pub struct CompMapList {
    maps: Vec<CompMap>,
}

impl CompMap {
    fn map_one(&self, x: &usize) -> Option<usize> {
        if self.from <= *x && *x < self.from + self.len {
            return Some(self.to + (*x - self.from));
        }
        None
    }
}

impl CompMapList {
    fn map_one(&self, x: &usize) -> usize {
        self.maps
            .iter()
            .map(|mp| mp.map_one(x))
            .find(|r| r.is_some())
            .or(Some(Some(*x)))
            .unwrap()
            .unwrap()
    }

    fn map_range(&self, (from, to): &(usize, usize)) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let mut x = *from;

        while x < *to {
            if let Some(mp) = self
                .maps
                .iter()
                .filter(|mp| x < mp.from + mp.len)
                .min_by_key(|mp| mp.from + mp.len)
            {
                if x < mp.from {
                    let y = min(*to, mp.from);
                    res.push((x, y));
                    x = y;
                }
                let y = min(*to, mp.from + mp.len);
                if x == y {
                    continue;
                }
                res.push((x - mp.from + mp.to, y - mp.from + mp.to));
                x = y;
            } else {
                res.push((x, *to));
                break;
            }
        }

        res
    }
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    seeds: Vec<usize>,
    maps: Vec<CompMapList>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();

    let seeds: Vec<usize> = lines
        .get(0)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    let maps: Vec<CompMapList> = raw
        .split("\n\n")
        .into_iter()
        .skip(1)
        .map(|block| CompMapList {
            maps: block
                .split("\n")
                .skip(1)
                .into_iter()
                .map(|line| {
                    let parts: Vec<usize> = line.split(" ").map(|d| d.parse().unwrap()).collect();
                    if let &[to, from, len] = &parts[..] {
                        return CompMap { from, to, len };
                    }
                    panic!("Unable to parse line")
                })
                .sorted_by(|cm1, cm2| cm1.from.cmp(&cm2.from))
                .collect(),
        })
        .collect();

    Input { lines, seeds, maps }
}

pub struct Day05;
impl Day for Day05 {
    fn day(&self) -> u8 {
        05
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let min: usize = input
            .seeds
            .iter()
            .map(|s| input.maps.iter().fold(*s, |id, mps| mps.map_one(&id)))
            .min()
            .unwrap();

        println!("Answer is {}", min);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let mut seeds: Vec<(usize, usize)> = input
            .seeds
            .chunks(2)
            .into_iter()
            .map(|a| match a {
                &[s, l] => (s, s + l),
                _ => panic!(),
            })
            .collect();

        for mps in input.maps {
            seeds = seeds.iter().flat_map(|s| mps.map_range(s)).collect();
        }

        println!("Answer is {}", seeds.iter().map(|s| s.0).min().unwrap());
    }
}
