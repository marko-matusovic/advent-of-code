use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashMap,
    iter::{once, repeat},
};

pub fn day() -> u8 {
    22
}

#[derive(Debug, Clone)]
enum Instruction {
    Walk(usize),
    Turn(i8),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Instruction::Turn(1),
            "L" => Instruction::Turn(-1),
            _ => match value.parse() {
                Ok(n) => Instruction::Walk(n),
                Err(_) => panic!(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    lines: Vec<String>,
    map: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
    cubes: HashMap<(usize, usize), HashMap<i8, (usize, usize, i8)>>,
}
impl Input {
    fn at_map(&self, pos: &Pos) -> char {
        self.map[pos.1][pos.0].clone()
    }
    fn print_map(&self, pos: &Option<Pos>) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(p) = pos {
                    if p.0 == x && p.1 == y {
                        print!("{}", p.dir_char());
                        continue;
                    }
                }
                print!("{}", cell.to_string());
            }
            println!()
        }
    }
}

#[derive(Debug, Clone)]
struct Pos(usize, usize, i8);
impl From<(usize, usize)> for Pos {
    fn from((a, b): (usize, usize)) -> Self {
        Pos(a, b, 0)
    }
}
impl Pos {
    fn add(&self, dir: (i8, i8)) -> Pos {
        Pos(
            (self.0 as isize + dir.0 as isize) as usize,
            (self.1 as isize + dir.1 as isize) as usize,
            self.2,
        )
    }
    fn sub(&self, dir: (i8, i8)) -> Pos {
        Pos(
            (self.0 as isize - dir.0 as isize) as usize,
            (self.1 as isize - dir.1 as isize) as usize,
            self.2,
        )
    }
    fn dir(&self) -> (i8, i8) {
        match self.2 {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => panic!(),
        }
    }
    fn dir_char(&self) -> char {
        match self.2 {
            0 => '>',
            1 => 'v',
            2 => '<',
            3 => '^',
            _ => panic!(),
        }
    }
    fn cube(&self) -> (usize, usize) {
        if self.0 == 0 || self.1 == 0 {
            return (100, 100); // whatever, just do something invalid
        }
        ((self.0 - 1) / 50, (self.1 - 1) / 50)
    }
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());

    let map_width = lines
        .iter()
        .take_while(|l| !l.is_empty())
        .map(|l| l.len())
        .max()
        .unwrap();

    let mut map: Vec<Vec<char>> = lines
        .iter()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            once(' ')
                .chain(l.chars())
                .chain(repeat(' '))
                .take(map_width + 2)
                .collect()
        })
        .collect();

    map.insert(0, repeat(' ').take(map_width + 2).collect());
    map.push(repeat(' ').take(map_width + 2).collect());

    let re = Regex::new(r"([RL])|(\d+)").unwrap();

    let instructions = re
        .captures_iter(lines.last().unwrap())
        .map(|cap| Instruction::from(cap.get(0).unwrap().as_str()))
        .collect_vec();

    // Manually fold the cubes, cause I'm going crazy
    //        (1, 0) (2, 0)
    //        (1, 1)
    // (0, 2) (1, 2)
    // (0, 3)
    let mut cubes = HashMap::new();
    let c = cubes.entry((2, 0)).or_insert_with(HashMap::new);
    c.insert(0, (1, 2, 2)); // ?
    c.insert(1, (1, 1, 2));
    c.insert(2, (1, 0, 2));
    c.insert(3, (0, 3, 3)); // ??
    let c = cubes.entry((1, 0)).or_insert_with(HashMap::new);
    c.insert(0, (2, 0, 0));
    c.insert(1, (1, 1, 1));
    c.insert(2, (0, 2, 0)); // ?
    c.insert(3, (0, 3, 0)); // ??
    let c = cubes.entry((1, 1)).or_insert_with(HashMap::new);
    c.insert(0, (2, 0, 3));
    c.insert(1, (1, 2, 1));
    c.insert(2, (0, 2, 1));
    c.insert(3, (1, 0, 3));
    let c = cubes.entry((1, 2)).or_insert_with(HashMap::new);
    c.insert(0, (2, 0, 2));
    c.insert(1, (0, 3, 2));
    c.insert(2, (0, 2, 2));
    c.insert(3, (1, 1, 3));
    let c = cubes.entry((0, 2)).or_insert_with(HashMap::new);
    c.insert(0, (1, 2, 0));
    c.insert(1, (0, 3, 1));
    c.insert(2, (1, 0, 0));
    c.insert(3, (1, 1, 0));
    let c = cubes.entry((0, 3)).or_insert_with(HashMap::new);
    c.insert(0, (1, 2, 3));
    c.insert(1, (2, 0, 1)); // ???
    c.insert(2, (1, 0, 1));
    c.insert(3, (0, 2, 3));

    dbg!(&cubes);

    Input {
        lines,
        map,
        instructions,
        cubes,
    }
}

fn init_pos(input: &Input) -> Pos {
    for y in 0..input.map.len() {
        for x in 0..input.map[y].len() {
            if input.at_map(&Pos(x, y, 0)) == '.' {
                return Pos(x, y, 0);
            }
        }
    }
    panic!()
}

fn move_wrap_line(input: &Input, pos: &Pos, ins: &Instruction) -> Pos {
    match ins {
        &Instruction::Turn(n) => Pos(pos.0, pos.1, (4 + pos.2 + n) % 4),
        &Instruction::Walk(steps) => {
            let dir: (i8, i8) = pos.dir();

            let mut cur: Pos = pos.clone();
            for _ in 0..steps {
                let next = cur.add(dir);
                match input.at_map(&next) {
                    '.' => {
                        cur = next;
                        continue;
                    }
                    '#' => return cur,
                    ' ' => {
                        let mut cur2 = cur.clone();
                        while input.at_map(&cur2) != ' ' {
                            cur2 = cur2.sub(dir);
                        }
                        if input.at_map(&cur2.add(dir)) == '#' {
                            return cur;
                        }
                        cur = cur2.add(dir);
                    }
                    _ => panic!(),
                }
            }

            cur
        }
    }
}

fn move_wrap_cube(input: &Input, pos: &Pos, ins: &Instruction) -> Pos {
    match ins {
        &Instruction::Turn(n) => Pos(pos.0, pos.1, (4 + pos.2 + n) % 4),
        &Instruction::Walk(steps) => {
            let mut cur: Pos = pos.clone();
            for _ in 0..steps {
                let next = cur.add(cur.dir());
                dbg!(&cur, &next);
                if next.cube() != cur.cube() {
                    dbg!(&cur, cur.cube(), &next, next.cube());
                    let (a, b, dir2) = input.cubes[&cur.cube()][&pos.2];
                    let x2 = 50 * a + 1;
                    let y2 = 50 * b + 1;
                    dbg!(&a, &b, &dir2, &x2, &y2);
                    let offset = match cur.2 {
                        0 => (cur.1 - 1) % 50,
                        1 => 49 - (cur.0 - 1) % 50,
                        2 => 49 - (cur.1 - 1) % 50,
                        3 => (cur.0 - 1) % 50,
                        _ => panic!(),
                    };
                    let next2 = match dir2 {
                        0 => Pos(x2, y2 + offset, dir2),
                        1 => Pos(x2 + 49 - offset, y2, dir2),
                        2 => Pos(x2 + 49, y2 + 49 - offset, dir2),
                        3 => Pos(x2 + offset, y2 + 49, dir2),
                        _ => panic!(),
                    };
                    dbg!(&cur, &next, &next2, &offset);
                    match input.at_map(&next2) {
                        '#' => return cur,
                        '.' => {
                            cur = next2.clone();
                            continue;
                        }
                        _ => panic!(),
                    }
                } else {
                    match input.at_map(&next) {
                        '.' => {
                            cur = next;
                            continue;
                        }
                        '#' => return cur,
                        // warping is handled above
                        _ => panic!(),
                    }
                }
            }

            cur
        }
    }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let init_pos = init_pos(&input);

    let fin_pos = input
        .instructions
        .iter()
        .fold(init_pos, |pos, ins| move_wrap_line(&input, &pos, ins));

    println!(
        "Answer is {}",
        fin_pos.1 * 1000 + fin_pos.0 * 4 + fin_pos.2 as usize
    );
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let init_pos = init_pos(&input);

    let fin_pos = input
        .instructions
        .iter()
        .fold(init_pos, |pos, ins| move_wrap_cube(&input, &pos, ins));

    println!(
        "Answer is {}",
        fin_pos.1 * 1000 + fin_pos.0 * 4 + fin_pos.2 as usize
    );

    println!("Answer is {}", 0);
}
