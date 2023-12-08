use itertools::Itertools;
use std::iter::once;

pub fn day() -> u8 {
    20
}

#[derive(Debug)]
struct Decryptor {
    values: Vec<isize>,
    link_right: Vec<usize>,
    link_left: Vec<usize>,
}

impl From<Vec<isize>> for Decryptor {
    fn from(vec: Vec<isize>) -> Self {
        let values = vec.clone();
        let link_right = (1..values.len()).into_iter().chain(once(0)).collect_vec();
        let link_left = once(values.len() - 1)
            .chain(0..values.len() - 1)
            .collect_vec();

        Decryptor {
            values,
            link_right,
            link_left,
        }
    }
}

impl Decryptor {
    fn mix(&mut self) {
        for (i_cur, v) in self.values.clone().iter().enumerate() {
            let wrap = self.values.len() as isize - 1;
            let skips = ((v % wrap) + wrap) % wrap;

            if skips == 0 {
                continue;
            }

            for _ in 0..skips.abs() {
                if skips > 0 {
                    let i_swap = self.link_right[i_cur];
                    self.swap(i_cur, i_swap);
                } else {
                    let i_swap = self.link_left[i_cur];
                    self.swap(i_swap, i_cur);
                };
            }
        }
    }
    fn collect(&self) -> isize {
        let find: Vec<usize> = [1000, 2000, 3000]
            .iter()
            .map(|i| i % self.values.len())
            .collect();
        let mut cur = self.values.iter().position(|v| *v == 0).unwrap();
        let mut total = 0;
        for i in 0..find.iter().max().unwrap().to_owned() + 1 {
            if find.contains(&i) {
                total += self.values[cur];
            }
            cur = self.link_right[cur];
        }
        return total;
    }

    fn print_list(&self) {
        print!("[");
        let mut cur = self.values.iter().position(|v| *v == 0).unwrap();
        for _ in 0..(self.values.len() - 1) {
            print!("{}, ", self.values[cur]);
            cur = self.link_right[cur];
        }
        println!("{}]", self.values[cur]);
    }

    fn swap(&mut self, a: usize, b: usize) {
        assert!(self.link_right[a] == b);
        assert!(self.link_left[b] == a);

        // BEFORE | left | a | b | right |
        // AFTER  | left | b | a | right |
        let left = self.link_left[a];
        let right = self.link_right[b];

        self.link_right[left] = b;
        self.link_right[b] = a;
        self.link_right[a] = right;

        self.link_left[right] = a;
        self.link_left[a] = b;
        self.link_left[b] = left;
    }
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    list: Vec<isize>,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());

    let list = lines.iter().map(|s| s.parse().unwrap()).collect();

    Input { lines, list }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let mut decryptor = Decryptor::from(input.list.clone());
    decryptor.mix();

    println!("Answer is {}", decryptor.collect());
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let mut decryptor = Decryptor::from(input.list.iter().map(|n| n * 811589153).collect_vec());
    for _ in 0..10 {
        decryptor.mix();
    }

    println!("Answer is {}", decryptor.collect());
}
