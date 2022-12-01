pub fn day() -> u8 {
    1
}

#[derive(Debug)]
pub struct Input {
    mass: Vec<u32>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines:Vec<&str> = raw.split("\n").collect();
    println!("Day {} parsing {} lines", day(), lines.len());
    let data: Vec<u32> = lines.iter().map(|&m| m.parse().unwrap()).collect();
    Input {
        mass: data,
    }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let total: u32 = input.mass.iter().map(|&m| ((m as f32)/3.0).floor() as u32 - 2).sum();

    println!("Answer is {}", total);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());
    
    let total: i32 = input.mass.iter().map(|&m| {
        let mut total = ((m as f32)/3.0).floor() as i32 - 2;
        let mut last = total.clone();
        loop {
            last = ((last as f32)/3.0).floor() as i32 - 2;
            if last > 0 {
                total += last;
            } else {
                break;
            }
        }
        total
    }).sum();

    println!("Answer is {}", total);
}
