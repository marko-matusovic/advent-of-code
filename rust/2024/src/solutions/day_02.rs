use pathfinding::num_traits::abs;

use super::day_trait::Day;

type Report = Vec<usize>;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    reports: Vec<Report>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    let reports = lines
        .iter()
        .map(|l| l.split(" ").map(|d| d.parse().unwrap()).collect())
        .collect();
    Input { lines, reports }
}

pub struct Day02;
impl Day for Day02 {
    fn day(&self) -> u8 {
        02
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let count: usize = input.reports.iter().filter(|&r| safe(r)).count();

        println!("Answer is {}", count);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let count: usize = input
            .reports
            .iter()
            .filter(|&r| {
                (0..r.len()).any(|i| {
                    let mut q = r.clone();
                    q.remove(i);
                    safe(&q)
                })
            })
            .count();

        println!("Answer is {}", count);
    }
}

fn safe(report: &Report) -> bool {
    let increasing = report[0] < report[1];
    report.windows(2).all(|w| {
        w[0] != w[1] && abs(w[0] as f32 - w[1] as f32) <= 3. && (w[0] < w[1]) == increasing
    })
}
