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
                let res: String = match digits.len() {
                    1 => digits.first().unwrap().to_string(),
                    2 => format!("{}{}", digits.first().unwrap(), digits.last().unwrap()),
                    _ => String::from("0"),
                };
                return res.parse::<i32>().unwrap();
            })
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        println!("Answer is {}", 0);
    }
}
