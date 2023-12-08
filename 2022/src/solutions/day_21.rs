use std::collections::HashMap;

pub fn day() -> u8 {
    21
}

#[derive(Debug, Clone)]
enum Op {
    Plus,
    Minus,
    Mult,
    Div,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Mult,
            "/" => Self::Div,
            _ => panic!("Invalid op: {}", value),
        }
    }
}
impl Op {
    fn apply(&self, a: isize, b: isize) -> isize {
        match self {
            Op::Plus => a + b,
            Op::Minus => a - b,
            Op::Mult => a * b,
            Op::Div => a / b,
        }
    }
    fn undo_left(&self, res: isize, a: isize) -> isize {
        match self {
            Op::Plus => res - a,
            Op::Mult => res / a,
            Op::Minus => a - res,
            Op::Div => a / res,
        }
    }
    fn undo_right(&self, res: isize, b: isize) -> isize {
        match self {
            Op::Plus => res - b,
            Op::Mult => res / b,
            Op::Minus => res + b,
            Op::Div => res * b,
        }
    }
}

#[derive(Debug, Clone)]
enum Monke {
    Knows(isize),
    Sees(String, String, Op),
    IsMe,
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    monkes: HashMap<String, Monke>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());

    let monkes = lines
        .iter()
        .map(|l| {
            let (name, job) = l.split_once(": ").unwrap();

            let monke = if let Ok(num) = job.parse::<isize>() {
                Monke::Knows(num)
            } else {
                let parts: Vec<&str> = job.split(" ").collect();
                Monke::Sees(parts[0].to_owned(), parts[2].to_owned(), Op::from(parts[1]))
            };
            return (name.to_owned(), monke);
        })
        .collect();

    Input { lines, monkes }
}

fn parse_monke(monkes: &HashMap<String, Monke>, monke: &Monke) -> Option<isize> {
    match monke {
        Monke::Knows(num) => Some(num.to_owned()),
        Monke::Sees(l, r, op) => {
            if let (Some(a), Some(b)) = (
                parse_monke(monkes, monkes.get(l).unwrap()),
                parse_monke(monkes, monkes.get(r).unwrap()),
            ) {
                Some(op.apply(a, b))
            } else {
                None
            }
        }
        Monke::IsMe => None,
    }
}
fn undo_find(monkes: &HashMap<String, Monke>, branch_with_me: &str, goal: isize) -> isize {
    return match monkes.get(branch_with_me).unwrap() {
        Monke::IsMe => goal,
        Monke::Knows(_) => panic!(),
        Monke::Sees(left, right, op) => {
            let left_value = parse_monke(&monkes, monkes.get(left).unwrap());
            let right_value = parse_monke(&monkes, monkes.get(right).unwrap());

            let (goal, branch_with_me) = if let Some(num) = left_value {
                (op.undo_left(goal, num), right)
            } else if let Some(num) = right_value {
                (op.undo_right(goal, num), left)
            } else {
                panic!()
            };

            return undo_find(monkes, branch_with_me, goal);
        }
    };
}
pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let num = parse_monke(&input.monkes, input.monkes.get("root").unwrap()).unwrap();

    println!("Answer is {}", num);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let (left, right) = input
        .monkes
        .get("root")
        .map(|m| match m {
            Monke::Sees(l, r, _) => (l, r),
            _ => panic!(),
        })
        .unwrap();

    let mut monkes = input.monkes.clone();
    monkes.insert("humn".to_owned(), Monke::IsMe);

    let left_value = parse_monke(&monkes, monkes.get(left).unwrap());
    let right_value = parse_monke(&monkes, monkes.get(right).unwrap());

    let (goal, branch_with_me) = if let Some(num) = left_value {
        (num, right)
    } else if let Some(num) = right_value {
        (num, left)
    } else {
        panic!()
    };

    let my_val = undo_find(&monkes, branch_with_me, goal);

    println!("Answer is {}", my_val);
}
