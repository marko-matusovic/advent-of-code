use std::collections::HashSet;

use priority_queue::PriorityQueue;

pub fn day() -> u8 {
    12
}

#[derive(Debug)]
pub struct Input {
    grid: Vec<Vec<usize>>,
    start: (usize, usize),
    end: (usize, usize),
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let grid: Vec<Vec<usize>> = lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, ch)| {
                    if ch == 'S' {
                        start = (x, y);
                        return 0;
                    }
                    if ch == 'E' {
                        end = (x, y);
                        return ('z' as usize) - ('a' as usize);
                    }
                    return (ch as usize) - ('a' as usize);
                })
                .collect()
        })
        .collect();
    Input { grid, start, end }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let grid = &input.grid;
    let start = &input.start;
    let end = &input.end;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: PriorityQueue<(usize, usize), i64> = PriorityQueue::new();
    queue.push(start.clone(), 0);

    let distance = loop {
        let (pos, dist) = queue.pop().unwrap();
        let dist = -dist;
        visited.insert(pos);
        if pos.0 == end.0 && pos.1 == end.1 {
            break dist;
        }
        let (x, y) = pos;
        let height = grid[y][x];
        if 1 <= x && grid[y][x - 1] <= height + 1 && !visited.contains(&(x - 1, y)) {
            queue.push((x - 1, y), -(dist + 1));
        }
        if 1 <= y && grid[y - 1][x] <= height + 1 && !visited.contains(&(x, y - 1)) {
            queue.push((x, y - 1), -(dist + 1));
        }
        if x + 1 < grid[y].len() && grid[y][x + 1] <= height + 1 && !visited.contains(&(x + 1, y)) {
            queue.push((x + 1, y), -(dist + 1));
        }
        if y + 1 < grid.len() && grid[y + 1][x] <= height + 1 && !visited.contains(&(x, y + 1)) {
            queue.push((x, y + 1), -(dist + 1));
        }
    };

    println!("Answer is {}", distance);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 1", day());

    let grid = &input.grid;
    let end = &input.end;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: PriorityQueue<(usize, usize), i64> = PriorityQueue::new();
    queue.push(end.clone(), 0);

    let distance = loop {
        let (pos, dist) = queue.pop().unwrap();
        let dist = -dist;
        visited.insert(pos);

        let (x, y) = pos;
        let height = grid[y][x];

        if height == 0 {
            break dist;
        }

        if 1 <= x && height <= grid[y][x - 1] + 1 && !visited.contains(&(x - 1, y)) {
            queue.push((x - 1, y), -(dist + 1));
        }
        if 1 <= y && height <= grid[y - 1][x] + 1 && !visited.contains(&(x, y - 1)) {
            queue.push((x, y - 1), -(dist + 1));
        }
        if x + 1 < grid[y].len() && height <= grid[y][x + 1] + 1 && !visited.contains(&(x + 1, y)) {
            queue.push((x + 1, y), -(dist + 1));
        }
        if y + 1 < grid.len() && height <= grid[y + 1][x] + 1 && !visited.contains(&(x, y + 1)) {
            queue.push((x, y + 1), -(dist + 1));
        }
    };

    println!("Answer is {}", distance);
}
