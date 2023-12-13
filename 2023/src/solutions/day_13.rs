use itertools::Itertools;

use super::day_trait::Day;

#[derive(Debug)]
struct Land {
    map: Vec<u128>,
    width: usize,
    height: usize,
}

impl Land {
    fn print(&self) {
        for h in 0..self.height {
            for w in 0..self.width {
                let w_bit: u128 = 1 << w;
                if self.map[h] & w_bit != 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn t(&self) -> Land {
        Land {
            width: self.height,
            height: self.width,
            map: (0..self.width)
                .map(|w| {
                    let old_bit: u128 = 1 << w;
                    (0..self.height)
                        .rev()
                        .map(|h| {
                            if self.map[h] & old_bit != 0 {
                                1 << h
                            } else {
                                0
                            }
                        })
                        .sum()
                })
                .collect_vec(),
        }
    }

    fn variants(&self) -> Vec<Land> {
        (0..self.height)
            .flat_map(|h| {
                (0..self.width)
                    .map(|w| {
                        let flip_bit: u128 = 1 << w;
                        Land {
                            width: self.width,
                            height: self.height,
                            map: self
                                .map
                                .iter()
                                .enumerate()
                                .map(|(r, row)| {
                                    if r == h {
                                        row.to_owned() ^ flip_bit
                                    } else {
                                        row.clone()
                                    }
                                })
                                .collect_vec(),
                        }
                    })
                    .collect::<Vec<Land>>()
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    lands: Vec<Land>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();

    let lands = raw
        .split("\n\n")
        .map(|block| {
            let lines = block.split("\n").collect_vec();
            Land {
                width: lines[0].len(),
                height: lines.len(),
                map: lines
                    .iter()
                    .map(|l| {
                        let mut n: u128 = 0;

                        l.char_indices().for_each(|(i, ch)| {
                            if ch == '#' {
                                n += 1 << i;
                            }
                        });

                        n
                    })
                    .collect(),
            }
        })
        .collect();

    Input { lines, lands }
}

fn find_horz_symetry(land: &Land) -> Vec<usize> {
    find_vert_symetry(&land.t())
}

fn find_vert_symetry(land: &Land) -> Vec<usize> {
    (0..land.height - 1)
        .filter(|&i| {
            let mut i = i;
            let mut j = i + 1;
            loop {
                if land.map[i] ^ land.map[j] != 0 {
                    return false;
                }
                if i == 0 || j + 1 == land.height {
                    return true;
                }
                i -= 1;
                j += 1;
            }
        })
        .map(|n| n + 1)
        .collect_vec()
}

pub struct Day13;
impl Day for Day13 {
    fn day(&self) -> u8 {
        13
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let sum: usize = input
            .lands
            .iter()
            .map(|land| {
                find_vert_symetry(land)
                    .iter()
                    .map(|n| n * 100)
                    .chain(find_horz_symetry(land).iter().map(|n| *n))
                    .collect_vec()[0]
            })
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        // // PRINT ALL VARIANTS
        // for l in input.lands {
        //     println!("Original");
        //     l.print();
        //     println!("Smugged");
        //     l.variants().iter().enumerate().for_each(|(i, v)|{
        //         println!("Variant {}", i);
        //         v.print();
        //     });
        // }

        let sum: usize = input
            .lands
            .iter()
            .map(|land| {
                let original = find_vert_symetry(land)
                    .iter()
                    .map(|n| n * 100)
                    .chain(find_horz_symetry(land).iter().map(|n| *n))
                    .collect_vec()[0];
                land.variants()
                    .iter()
                    .flat_map(|variant| {
                        find_vert_symetry(variant)
                            .iter()
                            .map(|n| n * 100)
                            .chain(find_horz_symetry(variant).iter().map(|n| *n))
                            .collect_vec()
                    })
                    .unique()
                    .find(|&n| n != original)
                    .unwrap_or(original)
            })
            .sum();

        println!("Answer is {}", sum);
    }
}
