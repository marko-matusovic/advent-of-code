use std::{cmp::min, collections::HashMap};

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    Input { lines: lines }
}

fn is_digit(c: &char) -> bool {
    return *c >= '0' && *c <= '9';
}

fn is_symbol(c: &char) -> bool {
    return !is_digit(c) && *c != '.';
}

fn check_star(
    input: &Input,
    x: usize,
    y: usize,
    star_map: &mut HashMap<String, Vec<usize>>,
    num: &usize,
) -> bool {
    if !(y < input.lines.len()) {
        return false;
    }
    let line = input.lines.get(y).unwrap();
    if !(x < line.len()) {
        return false;
    }
    if line.chars().nth(x).unwrap() != '*' {
        return false;
    }
    let key = format!("{}:{}", x, y);
    if !star_map.contains_key(&key) {
        star_map.insert(key.to_owned(), Vec::new());
    }
    star_map.get_mut(&key).unwrap().push(num.to_owned());
    return true;
}

pub struct Day03;
impl Day for Day03 {
    fn day(&self) -> u8 {
        03
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let mut sum = 0;

        for i in 0..input.lines.len() {
            let mut j = 0;
            let line = input.lines.get(i).unwrap();
            while j < line.len() {
                if !is_digit(&line.chars().nth(j).unwrap()) {
                    j += 1;
                    continue;
                }
                let mut k = j + 1;
                while k < line.len() && is_digit(&line.chars().nth(k).unwrap()) {
                    k += 1;
                }
                let num: usize = line[j..k].parse().unwrap();

                let mut touching = false;

                // Left
                if 0 < j && is_symbol(&line.chars().nth(j - 1).unwrap()) {
                    touching = true;
                }
                // Right
                if !touching && k < line.len() && is_symbol(&line.chars().nth(k).unwrap()) {
                    touching = true;
                }
                // Top (+ diagonals)
                if !touching
                    && 0 < i
                    && input.lines.get(i - 1).unwrap()[if j > 0 { j - 1 } else { 0 }
                        ..min(k + 1, input.lines.get(i - 1).unwrap().len())]
                        .chars()
                        .any(|c| is_symbol(&c))
                {
                    touching = true;
                }
                // Bottom (+ diagonals)
                if !touching
                    && i + 1 < input.lines.len()
                    && input.lines.get(i + 1).unwrap()[if j > 0 { j - 1 } else { 0 }
                        ..min(k + 1, input.lines.get(i + 1).unwrap().len())]
                        .chars()
                        .any(|c| is_symbol(&c))
                {
                    touching = true;
                }

                if touching {
                    sum += num;
                }
                j = k + 1;
            }
        }

        println!("Answer is {}", sum);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());
        let input: Input = parse_input(raw);

        let mut star_map: HashMap<String, Vec<usize>> = HashMap::new();

        for i in 0..input.lines.len() {
            let mut j = 0;
            let line = input.lines.get(i).unwrap();
            while j < line.len() {
                if !is_digit(&line.chars().nth(j).unwrap()) {
                    j += 1;
                    continue;
                }
                let mut k = j + 1;
                while k < line.len() && is_digit(&line.chars().nth(k).unwrap()) {
                    k += 1;
                }
                let num: usize = line[j..k].parse().unwrap();

                // Left
                if 0 < j {
                    check_star(&input, j - 1, i, &mut star_map, &num);
                }
                // Right
                check_star(&input, k, i, &mut star_map, &num);
                // Top (+ diagonals)
                for l in (if j > 0 { j - 1 } else { 0 })..(k + 1) {
                    if 0 < i {
                        check_star(&input, l, i - 1, &mut star_map, &num);
                    }
                    check_star(&input, l, i + 1, &mut star_map, &num);
                }

                j = k + 1;
            }
        }

        let sum: usize = star_map
            .iter()
            .filter(|(_k, v)| v.len() == 2)
            .map(|(_i, j)| j.first().unwrap() * j.last().unwrap())
            .sum();

        println!("Answer is {}", sum);
    }
}
