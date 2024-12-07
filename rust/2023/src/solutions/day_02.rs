use std::cmp::max;

use super::day_trait::Day;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Input {
    games: Vec<Game>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    Input {
        games: lines
            .iter()
            .map(|l| {
                let (id, draws) = l[5..].split_once(": ").expect("Invalid input!");
                Game {
                    id: id.parse().expect("Cannot parse id!"),
                    draws: draws
                        .split("; ")
                        .into_iter()
                        .map(|r| {
                            let mut draw = Draw {
                                red: 0,
                                green: 0,
                                blue: 0,
                            };
                            r.split(", ").for_each(|d| match d.split_once(" ") {
                                Some((n, "red")) => draw.red = n.parse().unwrap(),
                                Some((n, "green")) => draw.green = n.parse().unwrap(),
                                Some((n, "blue")) => draw.blue = n.parse().unwrap(),
                                _ => panic!(),
                            });

                            return draw;
                        })
                        .collect(),
                }
            })
            .collect(),
    }
}

pub struct Day02;
impl Day for Day02 {
    fn day(&self) -> u8 {
        02
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let sum: u32 = input
            .games
            .iter()
            .filter(|&g| {
                g.draws
                    .iter()
                    .all(|d| d.red <= 12 && d.green <= 13 && d.blue <= 14)
            })
            .map(|g| g.id)
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let sum: u32 = input.games.iter().map(|g| {
            let mut max_draw = Draw {
                red: 0,
                green: 0,
                blue: 0,
            };
            
            g.draws.iter().for_each(|round| {
                max_draw.red = max(max_draw.red, round.red);
                max_draw.green = max(max_draw.green, round.green);
                max_draw.blue = max(max_draw.blue, round.blue);
            });
            
            max_draw.red * max_draw.green * max_draw.blue
        }).sum();

        println!("Answer is {}", sum);
    }
}
