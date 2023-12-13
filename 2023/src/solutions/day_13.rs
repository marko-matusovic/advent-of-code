use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    maps: Vec<Vec<u128>>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();

    let maps: Vec<Vec<u128>> = raw
        .split("\n\n")
        .map(|block| {
            block
                .split("\n")
                .map(|l| {
                    let mut n = 0;

                    l.char_indices().for_each(|(i, ch)| {
                        if ch == '#' {
                            n += 1 << i;
                        }
                    });

                    n
                })
                .collect()
        })
        .collect();

    Input { lines, maps }
}

fn find_symetry(matrix: &Vec<u128>) -> Option<usize> {

    
    todo!()
}

fn t(matrix: &Vec<u128>) -> Vec<u128> {
    todo!()
}

pub struct Day13;
impl Day for Day13 {
    fn day(&self) -> u8 {
        13
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let sum: usize = input
            .maps
            .iter()
            .map(|mx| find_symetry(mx).unwrap_or_else(|| 100 * find_symetry(&t(mx)).unwrap()))
            .sum();

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let _input: Input = parse_input(raw);

        println!("Answer is {}", 0);
    }
}
