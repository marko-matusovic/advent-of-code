use std::collections::HashMap;

use itertools::Itertools;

pub fn day() -> u8 {
    17
}

#[derive(Debug)]
pub struct Input {
    shifts: String,
}

fn blocks() -> Vec<Vec<&'static str>> {
    // Upside down, because our tower is also upside down
    vec![
        vec!["####"],
        vec![".#.", "###", ".#."],
        vec!["###", "..#", "..#"], // only this is affected actually
        vec!["#", "#", "#", "#"],
        vec!["##", "##"],
    ]
}

pub fn parse_input(raw: &str) -> Input {
    Input {
        shifts: raw.to_owned(),
    }
}

fn hits(tower: &[[bool; 7]; 5000], block: &[&str], pos: &(usize, usize)) -> bool {
    if 7 <= pos.0 + block.get(0).unwrap().len() - 1 {
        return true;
    }

    let px: usize = pos.0.try_into().unwrap();
    let py: usize = pos.1.try_into().unwrap();
    for x in 0..block[0].len() {
        for y in 0..block.len() {
            if block[y].chars().nth(x).unwrap() == '#' && tower[py + y][px + x] {
                return true;
            }
        }
    }
    return false;
}

fn print_tower(tower: &[[bool; 7]; 5000]) {
    for row in tower {
        for &d in row {
            print!("{}", if d { '#' } else { '.' });
        }
        println!();
        if row.iter().all(|&d| !d) {
            break;
        }
    }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let mut tower: [[bool; 7]; 5000] = [[false; 7]; 5000];
    tower[0] = [true; 7];

    let mut max_height: usize = 0;

    let mut sf: usize = 0;

    let all_blocks = blocks().to_owned();

    for i in 0..2022 {
        let block = all_blocks.get(i % 5).unwrap();

        // pos points to left top
        let mut pos: (usize, usize) = (2, max_height + 4);

        loop {
            // println!("pos ({}, {})", pos.0, pos.1);
            // shift < or >
            let shift = input.shifts.chars().nth(sf);
            sf = (sf + 1) % input.shifts.len();

            let new_pos = match shift {
                Some('<') => (if pos.0 == 0 { 0 } else { pos.0 - 1 }, pos.1),
                Some('>') => (pos.0 + 1, pos.1),
                _ => panic!("Unexpected shift"),
            };

            if !hits(&tower, &block, &new_pos) {
                pos = new_pos;
            }

            // shift v

            if !hits(&tower, &block, &(pos.0, pos.1 - 1)) {
                pos.1 -= 1;
                continue;
            }

            // movement down hit, place in tower
            for x in 0..block[0].len() {
                for y in 0..block.len() {
                    if block[y].chars().nth(x).unwrap() == '#' {
                        tower[pos.1 + y][pos.0 + x] = true;
                    }
                }
            }

            max_height = tower
                .iter()
                .find_position(|&r| r.iter().all(|&x| !x))
                .unwrap()
                .0 - 1;

            // proceed to next block
            break;
        }
    }

    println!("Answer is {}", max_height);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let mut tower: [[bool; 7]; 5000] = [[false; 7]; 5000];
    tower[0] = [true; 7];

    let mut max_height: usize = 0;
    let mut shifted_height: usize = 0;

    let mut shift_i: usize = 0;

    let all_blocks = blocks().to_owned();

    let mut states: HashMap<([usize; 7], usize, usize), (usize, usize)> = HashMap::new();

    let mut i = 0;
    while i < 1000000000000 {
        let block_i = i % 5;
        let block = all_blocks.get(block_i).unwrap();
        i += 1;

        // pos points to left top
        let mut pos: (usize, usize) = (2, max_height + 4);

        loop {
            // println!("pos ({}, {})", pos.0, pos.1);
            // shift < or >
            let shift = input.shifts.chars().nth(shift_i);
            shift_i = (shift_i + 1) % input.shifts.len();

            let new_pos = match shift {
                Some('<') => (if pos.0 == 0 { 0 } else { pos.0 - 1 }, pos.1),
                Some('>') => (pos.0 + 1, pos.1),
                _ => panic!("Unexpected shift"),
            };

            if !hits(&tower, &block, &new_pos) {
                pos = new_pos;
            }

            // shift v

            if !hits(&tower, &block, &(pos.0, pos.1 - 1)) {
                pos.1 -= 1;
                continue;
            }

            // movement down hit
            // proceed to next block
            break;
        }

        // place in tower
        for x in 0..block[0].len() {
            for y in 0..block.len() {
                if block[y].chars().nth(x).unwrap() == '#' {
                    tower[pos.1 + y][pos.0 + x] = true;
                }
            }
        }
        
        // find new max height
        max_height = tower
            .iter()
            .find_position(|&r| r.iter().all(|&x| !x))
            .unwrap()
            .0 - 1;

        // shift down if running close to end
        if 4900 < max_height {
            let mut new_tower = [[false; 7]; 5000];
            for (i, row) in tower[4800..5000].iter().enumerate() {
                new_tower[i] = *row;
            }
            tower = new_tower;
            max_height -= 4800;
            shifted_height += 4800;
        }

        // finding loop
        let mut cols = [0; 7];
        for c in 0..7 {
            cols[c] = tower.iter().rposition(|d| d[c]).unwrap()
        }
        let min_c = cols.iter().min().unwrap().to_owned();
        for c in 0..7 {
            cols[c] -= min_c;
        }

        let key = (cols, block_i, shift_i);
        
        if let Some(&(prev_i, prev_h )) = states.get(&key) {
            let loop_size = i - prev_i;
            let loop_height = shifted_height + max_height - prev_h;
            while i + loop_size < 1000000000000 {
                i += loop_size;
                shifted_height += loop_height;
            }
        } else {
            states.insert(key, (i, shifted_height + max_height));
        }
    }

    println!("Answer is {}", shifted_height + max_height);
}

// 1597714284667 too low
// 1597714285698 too low