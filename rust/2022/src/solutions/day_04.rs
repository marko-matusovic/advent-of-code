pub fn day() -> u8 {
    4
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<((i32, i32), (i32, i32))>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<&str> = raw.split("\n").collect();
    println!("Day {} parsing {} lines", day(), lines.len());
    Input {
        lines: lines
            .iter()
            .map(|&l| {
                l.split_once(",")
                    .map(|(l, r)| {
                        (
                            l.split_once("-")
                                .map(|(m, n)| (m.parse().unwrap(), n.parse().unwrap()))
                                .unwrap(),
                            r.split_once("-")
                                .map(|(m, n)| (m.parse().unwrap(), n.parse().unwrap()))
                                .unwrap(),
                        )
                    })
                    .unwrap()
            })
            .collect(),
    }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let total = input
        .lines
        .iter()
        .filter(|&((a, b), (c, d))| 
            (a <= c && d <= b) || 
            (c <= a && b <= d))
        .count();

    println!("Answer is {}", total);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let total = input
        .lines
        .iter()
        .filter(|&((a, b), (c, d))|
            (a <= c && c <= b) || 
            (a <= d && d <= b) || 
            (c <= a && a <= d) || 
            (c <= b && b <= d))
        .count();

    println!("Answer is {}", total);
}
