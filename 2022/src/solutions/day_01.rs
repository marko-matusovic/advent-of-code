pub fn day() -> u8 {
    1
}

#[derive(Debug)]
pub struct Input {
    elves: Vec<Vec<i32>>,
}

pub fn parse_input(raw: &str) -> Input {
    let elves = raw.split("\n\n");
    let data: Vec<Vec<i32>> = elves
        .into_iter()
        .map(|elf| {
            elf.split("\n")
                .into_iter()
                .map(|food| food.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    println!("Day {} parsing {} lines", day(), data.len());
    Input { elves: data }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());
    let max_total: i32 = input
        .elves
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .unwrap();
    println!("Answer is {}", max_total);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());
    let mut totals: Vec<i32> = input.elves.iter().map(|elf| elf.iter().sum()).collect();
    totals.sort();
    totals.reverse();
    let sum: i32 = totals[0..3].iter().sum();
    println!("Answer is {}", sum);
}
