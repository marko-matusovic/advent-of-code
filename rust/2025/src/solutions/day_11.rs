use super::day_trait::Day;
use itertools::Itertools;
use pathfinding::prelude::count_paths;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
    edges: HashMap<String, Vec<String>>,
}

fn parse_input(raw: &str) -> Input {
    Input {
        edges: raw
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(a, b)| (a.to_string(), b.split(' ').map(|s| s.to_string()).collect()))
            .collect(),
    }
}

pub struct Day11;
impl Day for Day11 {
    fn day(&self) -> u8 {
        11
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let edges = parse_input(raw).edges;

        let count = count_paths(
            "you".to_string(),
            |cur| edges.get(cur).unwrap().iter().map(|s| s.to_string()),
            |end| end == "out",
        );

        println!("Answer is {}", count);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let edges = parse_input(raw).edges;

        // svr - dac - fft - out
        //     \ fft - dac /

        let svr_dac = count(&edges, "svr", "dac");
        let dac_fft = count(&edges, "dac", "fft");
        let fft_out = count(&edges, "fft", "out");

        let svr_fft = count(&edges, "svr", "fft");
        let fft_dac = count(&edges, "fft", "dac");
        let dac_out = count(&edges, "dac", "out");

        println!(
            "Answer is {}",
            svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out
        );
    }
}

fn count(edges: &HashMap<String, Vec<String>>, from: &str, to: &str) -> usize {
    count_paths(
        from.to_string(),
        |cur| {
            edges
                .get(cur)
                .unwrap_or(&Vec::new())
                .iter()
                .map(|s| s.to_string())
                .collect_vec()
        },
        |end| end == to,
    )
}
