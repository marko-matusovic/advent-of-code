pub fn day() -> u8 {
    25
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
}

struct SNAFU {
    display: String,
    number: usize,
}
impl SNAFU {
    fn to_dec(value: &str) -> usize {
        value
            .chars()
            .map(|ch| match ch {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!(),
            })
            .fold(0, |acc, cur| ((acc * 5) as isize + cur) as usize)
    }
    fn from_dec(value: usize) -> String {
        let md = |n: usize| ((n + 2) % 5) as isize - 2;
        let dv = |n:usize| (n + 2) / 5;

        let mut v = value;
        let mut parsed = String::new();
        while v != 0 {
            let ch = match md(v) {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => panic!(),
            };
            parsed = format!("{}{}", ch, parsed);
            v = dv(v);
        }

        parsed
    }
}

impl From<&str> for SNAFU {
    fn from(value: &str) -> Self {
        SNAFU {
            display: value.to_owned(),
            number: SNAFU::to_dec(value),
        }
    }
}

impl From<usize> for SNAFU {
    fn from(value: usize) -> Self {
        SNAFU {
            display: SNAFU::from_dec(value),
            number: value.to_owned(),
        }
    }
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());
    Input { lines }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let sum: usize = input
        .lines
        .iter()
        .map(|l| SNAFU::to_dec(l.as_str()))
        .sum();

    println!("{}", sum);

    let parsed = SNAFU::from_dec(sum);

    println!("Answer is {}", parsed);
}

pub fn part_2(_input: &Input) {
    println!("Las day part 2 is FREE!");
}
