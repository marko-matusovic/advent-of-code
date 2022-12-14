use std::cmp::{max, min};

use matrix::{
    self,
    prelude::{compressed::Variant, Compressed},
};

pub fn day() -> u8 {
    14
}

#[derive(Debug)]
pub struct Input {
    rocks: Vec<Vec<(usize, usize)>>,
}

enum Thing {
    AIR = 0,
    ROCK = 1,
    SAND = 2,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());

    let rocks = lines
        .iter()
        .map(|l| {
            l.split(" -> ")
                .map(|p| {
                    let (x, y) = p.split_once(",").unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect();

    Input { rocks }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let tap = (500, 0);
    let mut grid: Compressed<u8> = generate_grid(&input.rocks);
    let max_y = input
        .rocks
        .iter()
        .map(|rl| rl.iter().map(|r| r.1).max().unwrap())
        .max()
        .unwrap();

    let mut sand_count = 0;
    'generate: loop {
        // generate new sand
        let mut sand = tap.clone();
        loop {
            // move sand
            if sand.1 > max_y {
                break 'generate;
            } else if grid.get((sand.0, sand.1 + 1)) == 0 {
                sand = (sand.0, sand.1 + 1);
                continue;
            } else if grid.get((sand.0 - 1, sand.1 + 1)) == 0 {
                sand = (sand.0 - 1, sand.1 + 1);
                continue;
            } else if grid.get((sand.0 + 1, sand.1 + 1)) == 0 {
                sand = (sand.0 + 1, sand.1 + 1);
                continue;
            } else {
                grid.set(sand, 2);
                break;
            }
        }
        sand_count += 1;
    }

    println!("Answer is {}", sand_count);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let tap = (500, 0);
    let max_x = input
        .rocks
        .iter()
        .map(|rl| rl.iter().map(|r| r.0).max().unwrap())
        .max()
        .unwrap();
    let max_y = input
        .rocks
        .iter()
        .map(|rl| rl.iter().map(|r| r.1).max().unwrap())
        .max()
        .unwrap();
    // let min_x = input
    //     .rocks
    //     .iter()
    //     .map(|rl| rl.iter().map(|r| r.0).min().unwrap())
    //     .min()
    //     .unwrap();
    // let min_y = input
    //     .rocks
    //     .iter()
    //     .map(|rl| rl.iter().map(|r| r.1).min().unwrap())
    //     .min()
    //     .unwrap();

    let mut rocks_with_floor = input.rocks.clone();
    rocks_with_floor.push(vec![(0, max_y + 2), (max_x + max_y, max_y + 2)]);

    let mut grid: Compressed<u8> = generate_grid(&rocks_with_floor);

    let mut sand_count = 0;
    loop {
        // generate new sand

        // println!("grid:");
        // for y in min_y - 3..=max_y + 3 {
        //     for x in min_x - 3..=max_x + 3 {
        //         print!(
        //             "{}",
        //             match grid.get((x, y)) {
        //                 0 => ".",
        //                 1 => "#",
        //                 2 => "o",
        //                 _ => panic!("bad value in grid"),
        //             }
        //         );
        //     }
        //     println!();
        // }

        if grid.get(tap) != 0 {
            break;
        }
        let mut sand = tap.clone();
        loop {
            // move sand
            if sand.1 > max_y + 2 {
                panic!("oops, floor is not good.")
            } else if grid.get((sand.0, sand.1 + 1)) == 0 {
                sand = (sand.0, sand.1 + 1);
                continue;
            } else if grid.get((sand.0 - 1, sand.1 + 1)) == 0 {
                sand = (sand.0 - 1, sand.1 + 1);
                continue;
            } else if grid.get((sand.0 + 1, sand.1 + 1)) == 0 {
                sand = (sand.0 + 1, sand.1 + 1);
                continue;
            } else {
                grid.set(sand, 2);
                break;
            }
        }
        sand_count += 1;
    }

    println!("Answer is {}", sand_count);
}

fn generate_grid(rocks: &Vec<Vec<(usize, usize)>>) -> Compressed<u8> {
    let max_x = rocks
        .iter()
        .map(|rl| rl.iter().map(|r| r.0).max().unwrap())
        .max()
        .unwrap();
    let max_y = rocks
        .iter()
        .map(|rl| rl.iter().map(|r| r.1).max().unwrap())
        .max()
        .unwrap();

    let mut grid: Compressed<u8> = Compressed::new((max_x + 5, max_y + 5), Variant::Row);

    for rock in rocks {
        rock.windows(2).for_each(|a| {
            let (x0, y0) = a[0];
            let (x1, y1) = a[1];
            if x0 == x1 {
                for y in min(y0, y1)..=max(y0, y1) {
                    grid.set((x0, y), 1);
                }
            } else if y0 == y1 {
                for x in min(x0, x1)..=max(x0, x1) {
                    grid.set((x, y0), 1);
                }
            } else {
                panic!("bad rocks!");
            }
        })
    }

    return grid;
}
