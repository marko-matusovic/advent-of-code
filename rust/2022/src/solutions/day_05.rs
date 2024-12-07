use std::collections::HashMap;

pub fn day() -> u8 {
    5
}

#[derive(Debug)]
pub struct Input {
    stacks: HashMap<usize, Vec<char>>,
    instructions: Vec<(usize, usize, usize)>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<&str> = raw.split("\n").collect();
    println!("Day {} parsing {} lines", day(), lines.len());

    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    let mut it = lines.iter();
    loop {
        let &line = it.next().unwrap();
        if line.contains(" 1 ") {
            break;
        }
        let mut i: usize = 1;
        loop {
            let pos = (i * 4) - 3;
            if let Some(ch) = line.chars().nth(pos) {
                if ch != ' ' {
                    stacks.entry(i).or_insert(Vec::new()).push(ch)
                }
            } else {
                break;
            }
            i += 1
        }
    }

    stacks.iter_mut().for_each(|(_, values)| values.reverse());

    let mut instructions: Vec<(usize, usize, usize)> = Vec::new();

    it.next(); // skip empty line

    loop {
        if let Some(line) = it.next() {
            let parts: Vec<&str> = line.split(" ").into_iter().collect();

            instructions.push((
                parts.get(1).unwrap().parse().unwrap(),
                parts.get(3).unwrap().parse().unwrap(),
                parts.get(5).unwrap().parse().unwrap(),
            ))
        } else {
            break;
        }
    }

    Input {
        stacks,
        instructions,
    }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let mut stacks = input.stacks.clone();
    let instructions = input.instructions.clone();

    for (n, a, b) in instructions {
        for _ in 0..n {
            let val = stacks.get_mut(&a).unwrap().pop();
            stacks.get_mut(&b).unwrap().push(val.unwrap());
        }
    }

    let n = stacks.len();

    let mut res = String::new();
    for i in 1..=n {
        res = format!("{}{}", res, stacks.get(&i).unwrap().last().unwrap().to_string());
    }

    println!("Answer is {}", res);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let mut stacks = input.stacks.clone();
    let instructions = input.instructions.clone();

    for (n, a, b) in instructions {
        let mut temp: Vec<char> = Vec::new();
        let sa = stacks.get_mut(&a).unwrap();
        for _ in 0..n {
            temp.push(sa.pop().unwrap());
        }
        let sb = stacks.get_mut(&b).unwrap();
        for _ in 0..n {
            sb.push(temp.pop().unwrap());
        }
    }

    let n = stacks.len();

    let mut res = String::new();
    for i in 1..=n {
        res = format!("{}{}", res, stacks.get(&i).unwrap().last().unwrap().to_string());
    }

    println!("Answer is {}", res);
}
