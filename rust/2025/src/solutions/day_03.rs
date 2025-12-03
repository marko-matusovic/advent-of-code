use super::day_trait::Day;
use pathfinding::num_traits::pow;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    Input { lines }
}

pub struct Day03;
impl Day for Day03 {
    fn day(&self) -> u8 {
        03
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let max_jolt = input
            .lines
            .iter()
            .map(|l| {
                find_max_jolt(l, 2)
                // let (i, s1) = l[..l.len() - 1]
                //     .chars()
                //     .enumerate()
                //     .min_by(|(_, a), (_, b)| a.cmp(b).reverse())
                //     .unwrap();
                // let s2 = l[i + 1..].chars().max().unwrap();
                // let d1: usize = s1.to_string().parse().unwrap();
                // let d2: usize = s2.to_string().parse().unwrap();
                // 10 * d1 + d2
            })
            .sum::<usize>();

        println!("Answer is {}", max_jolt);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let max_jolt = input
            .lines
            .iter()
            .map(|l| find_max_jolt(l, 12))
            .sum::<usize>();

        println!("Answer is {}", max_jolt);
    }
}

fn find_max_jolt(bank: &str, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    if bank.len() < len {
        dbg!(bank, len);
        panic!("Not enough batteries in bank!");
    }
    let (i, s) = bank[..bank.len() - len + 1]
        .chars()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b).reverse())
        .unwrap();
    let d: usize = s.to_string().parse().unwrap();
    d * pow(10, len - 1) + find_max_jolt(&bank[i + 1..], len - 1)
}
