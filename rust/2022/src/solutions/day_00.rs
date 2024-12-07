use std::collections::HashMap;

pub fn day() -> u8 {
    0
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines = raw.split("\n");
    println!("Day {} parsing {} lines", day(), raw.split("\n").count());
    Input {
        lines: lines.map(|l| String::from(l)).collect(),
    }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1 with {:?}", day(), &input);

    let mut res = "".to_owned();

    let mut prev;
    if let Some(val) = input.lines.first() {
        prev = val;
        res.push_str(&val);

        for curr in input.lines[1..].iter() {
            let a = prev.as_bytes();
            let b = curr.as_bytes();

            let mut aa: HashMap<u8, u8> = HashMap::new();
            let mut bb: HashMap<u8, u8> = HashMap::new();

            a.iter().for_each(|&ai| {
                if let Some(val) = aa.get(&ai) {
                    aa.insert(ai, val + 1);
                } else {
                    aa.insert(ai, 1);
                }
            });

            b.iter().for_each(|&bi| {
                if let Some(val) = bb.get(&bi) {
                    bb.insert(bi, val + 1);
                } else {
                    bb.insert(bi, 1);
                }
            });

            for &key in bb.keys() {
                if !aa.contains_key(&key) || aa.get(&key) != bb.get(&key) {
                    res.push_str(&char::from(key).to_string());
                    break;
                }
            }

            prev = curr;
        }

        println!("Answer: {}", &res);
    } else {
        println!("Bad")
    }
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2 with {:?}", day(), &input);

    println!("Answer");
}
