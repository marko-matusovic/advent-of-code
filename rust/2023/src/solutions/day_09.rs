use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    history: Vec<Vec<isize>>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let history = lines
        .iter()
        .map(|l| l.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect();
    Input { lines, history }
}

fn next_value(sequence: &Vec<isize>) -> isize {
    if sequence.iter().all(|&n| n == 0) {
        return 0;
    }

    let subseq: Vec<isize> = sequence
        .windows(2)
        .map(|w| match w {
            &[a, b] => b - a,
            _ => panic!(),
        })
        .collect();

    return sequence.last().unwrap() + next_value(&subseq);
}

pub struct Day09;
impl Day for Day09 {
    fn day(&self) -> u8 {
        09
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let sum: isize = input.history.iter().map(|s| next_value(s)).sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let sum: isize = input
            .history
            .iter()
            .map(|s| s.iter().map(|&n| 0 - n).rev().collect::<Vec<isize>>())
            .map(|s| next_value(&s))
            .map(|v| 0 - v)
            .sum();

        println!("Answer is {}", sum);
    }
}
