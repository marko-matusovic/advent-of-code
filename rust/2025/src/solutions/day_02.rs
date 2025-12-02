use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    ranges: Vec<(u64, u64)>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split(",").map(|s| s.to_owned()).collect();
    Input {
        lines: lines.clone(),
        ranges: lines
            .iter()
            .map(|s| match s.split_once("-") {
                Some((from, to)) => (from.parse::<u64>().unwrap(), to.parse::<u64>().unwrap()),
                __ => panic!("Unexpected input format!"),
            })
            .collect(),
    }
}

pub struct Day02;
impl Day for Day02 {
    fn day(&self) -> u8 {
        02
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let mut sum_ids = 0;

        for (from, to) in input.ranges {
            // dbg!(from, to);
            for id in from..=to {
                if symmetry(&id.to_string(), 2) {
                    // dbg!(&sid);
                    sum_ids += id;
                }
            }
        }

        println!("Answer is {}", sum_ids);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let mut sum_ids = 0;

        for (from, to) in input.ranges {
            'id: for id in from..=to {
                let sid = id.to_string();
                for c in 2..=sid.len() {
                    if symmetry(&sid, c) {
                        sum_ids += id;
                        continue 'id;
                    }
                }
            }
        }

        println!("Answer is {}", sum_ids);
    }
}

fn symmetry(sid: &String, count: usize) -> bool {
    if count <= 1 {
        panic!("Invalid count!");
    }
    if sid.len() % count != 0 {
        return false;
    }
    let size = sid.len() / count;
    for i in 0..count - 1 {
        if sid[i * size..(i + 1) * size] != sid[(i + 1) * size..(i + 2) * size] {
            return false;
        }
    }
    true
}
