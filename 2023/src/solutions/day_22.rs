pub fn day() -> u8 {
    22
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());
    Input { lines: lines }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    println!("Answer is {}", 0);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    println!("Answer is {}", 0);
}
