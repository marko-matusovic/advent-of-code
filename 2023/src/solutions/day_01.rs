use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    Input { lines: lines }
}

fn is_digit(c: &char) -> bool {
    return *c >= '0' && *c <= '9';
}

fn str_to_digit(s: &str) -> i32 {
    return match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => -1,
    };
}

pub struct Day01;
impl Day for Day01 {
    fn day(&self) -> u8 {
        01
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let sum: i32 = input
            .lines
            .iter()
            .map(|ln| {
                let digits: Vec<char> = ln.chars().filter(|c| is_digit(c)).collect();
                let res: String = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
                return res.parse::<i32>().unwrap();
            })
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let sum: i32 = input
            .lines
            .iter()
            .map(|ln| {
                let mut digits: Vec<i32> = Vec::new();
                let mut i = 0;
                while i < ln.len() {
                    for j in [1, 3, 4, 5] {
                        if ln.len() < i + j {
                            break;
                        }
                        let d = str_to_digit(&ln[i..i + j]);
                        if 0 < d {
                            digits.push(d);
                            break;
                        }
                    }
                    i += 1;
                }
                let res: String = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
                return res.parse::<i32>().unwrap();
            })
            .sum();

        println!("Answer is {}", sum);
    }
}
