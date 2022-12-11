pub fn day() -> u8 {
    10
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Instruction {
    ADDX(i32),
    NOOP,
}

#[derive(Debug)]
pub struct Input {
    instructions: Vec<Instruction>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());

    let instructions: Vec<Instruction> = lines
        .iter()
        .map(|l| {
            if l.starts_with("noop") {
                Instruction::NOOP
            } else {
                Instruction::ADDX(l.split_at(5).1.parse().unwrap())
            }
        })
        .collect();

    Input { instructions }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let instructions = input.instructions.clone();

    let mut x: i64 = 1;
    let mut x_over_cycles: Vec<i64> = vec![0];

    instructions.iter().for_each(|&ins| {
        x_over_cycles.push(x);
        if let Instruction::ADDX(num) = ins {
            x += num as i64;
            x_over_cycles.push(x);
        }
    });

    let score_over_cycles: Vec<i64> = x_over_cycles
        .iter()
        .enumerate()
        .map(|(i, &v)| v*(1 + i as i64))
        .collect();

    let total: i64 = score_over_cycles
        .iter()
        .enumerate()
        .filter(|&(i, _v)| (i as i64 - 19) % 40 == 0)
        .map(|(_i, v)| v)
        .sum();

    println!("Answer is {}", total);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());
    let instructions = input.instructions.clone();

    let mut x: i64 = 1;
    let mut x_over_cycles: Vec<i64> = vec![0];

    instructions.iter().for_each(|&ins| {
        x_over_cycles.push(x);
        if let Instruction::ADDX(num) = ins {
            x += num as i64;
            x_over_cycles.push(x);
        }
    });

    let mut chars: Vec<char> = Vec::new();
    let mut i: usize = 0;

    for pos in x_over_cycles {
        if pos + 1 < 0 {
            continue;
        }
        if pos - 1 <= i as i64 && i as i64 <= pos + 1 {
            chars.push('#');
        } else {
            chars.push('.');
        }
        i += 1;
        i = i%40;
    }

    println!("Answer is:");
    chars.chunks(40).for_each(|chunk| println!("{}", chunk.iter().collect::<String>()));

}
