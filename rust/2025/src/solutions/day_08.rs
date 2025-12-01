use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    Input { lines }
}

pub struct Day08;
impl Day for Day08 {
    fn day(&self) -> u8 {
        08
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let _input: Input = parse_input(raw);

        println!("Answer is {}", 0);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let _input: Input = parse_input(raw);

        println!("Answer is {}", 0);
    }
}
