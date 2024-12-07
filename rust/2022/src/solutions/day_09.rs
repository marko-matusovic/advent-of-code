use std::{collections::HashSet, hash::Hash, ops::Add};

pub fn day() -> u8 {
    9
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: char,
    distance: u32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
pub struct Input {
    instructions: Vec<Instruction>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());

    let instructions = lines
        .iter()
        .map(|l| {
            let (dir, dist) = l.split_once(" ").unwrap();
            Instruction {
                direction: dir.chars().nth(0).unwrap(),
                distance: dist.parse().unwrap(),
            }
        })
        .collect();

    Input {
        instructions: instructions,
    }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    let mut tail_visited: HashSet<Pos> = HashSet::new();
    let instructions = input.instructions.clone();

    for Instruction {
        direction,
        distance,
    } in instructions
    {
        let delta = decode_direction(&direction);
        for _ in 0..distance {
            head = head + delta.clone();
            tail = move_tail(&head, &tail);
            tail_visited.insert(tail.to_owned());
        }
    }

    println!("Answer is {}", tail_visited.len());
    // 6389 too low
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());
    let mut tail_visited: HashSet<Pos> = HashSet::new();
    let instructions = input.instructions.clone();

    let mut rope: [Pos; 10] = [Pos { x: 0, y: 0 }; 10];

    for Instruction {
        direction,
        distance,
    } in instructions
    {
        let delta = decode_direction(&direction);
        for _ in 0..distance {
            rope[0] = rope[0] + delta.clone();
            for i in 0..=8 {
                rope[i + 1] = move_tail(&rope[i], &rope[i + 1]);
            }
            tail_visited.insert(rope[9].to_owned());
        }
    }

    println!("Answer is {}", tail_visited.len());
}

fn decode_direction(direction: &char) -> Pos {
    match direction {
        'U' => Pos { x: 0, y: 1 },
        'D' => Pos { x: 0, y: -1 },
        'R' => Pos { x: 1, y: 0 },
        'L' => Pos { x: -1, y: 0 },
        d => panic!("Unexpected direction, '{}'", d),
    }
}

fn move_tail(head: &Pos, tail: &Pos) -> Pos {
    if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
        return tail.to_owned();
    }

    let mut dx = 0;
    if head.x - tail.x > 0 {
        dx = 1;
    } else if head.x - tail.x < 0 {
        dx = -1;
    }

    let mut dy = 0;
    if head.y - tail.y > 0 {
        dy = 1;
    } else if head.y - tail.y < 0 {
        dy = -1;
    }

    return Pos {
        x: tail.x + dx,
        y: tail.y + dy,
    };
}
