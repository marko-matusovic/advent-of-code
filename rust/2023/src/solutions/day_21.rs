use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    ops::Add,
};

use crate::libs::{dir_2d::Dir4, Pos2I, Pos2U};

use super::day_trait::Day;

#[derive(Debug)]
pub struct Input {
    lines: Vec<String>,
    rocks: HashSet<Pos2I>,
    start: Pos2I,
    grid_size: Pos2U,
}

fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();

    let h = lines.len();
    let w = lines[0].len();

    let rocks = lines
        .iter()
        .enumerate()
        .map(move |(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(x, _)| Pos2U(x, y).into())
        })
        .flatten()
        .collect();
    let start = lines
        .iter()
        .enumerate()
        .flat_map(move |(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == 'S')
                .map(move |(x, _)| Pos2U(x, y).into())
        })
        .find(|_| true)
        .unwrap();
    Input {
        lines,
        rocks,
        start,
        grid_size: Pos2U(w, h),
    }
}

pub struct Day21;
impl Day for Day21 {
    fn day(&self) -> u8 {
        21
    }

    fn part_1(&self, raw: &str) {
        println!("Day {} part 1", self.day());
        let input: Input = parse_input(raw);

        let w = input.grid_size.0;
        let h = input.grid_size.1;
        let rocks: HashSet<Pos2I> = input
            .rocks
            .iter()
            .map(|Pos2I(x, y)| Pos2I(x + 1, y + 1))
            .chain((0..=w + 1).into_iter().map(|x| Pos2U(x, 0).into()))
            .chain((0..=w + 1).into_iter().map(|x| Pos2U(x, h + 1).into()))
            .chain((0..=h + 1).into_iter().map(|y| Pos2U(0, y).into()))
            .chain((0..=h + 1).into_iter().map(|y| Pos2U(w + 1, y).into()))
            .collect();

        let count = count_possible_after(
            &(input.grid_size + Pos2U(1, 1)),
            &(input.start + Pos2I(1, 1)),
            &rocks,
            64,
        );

        println!("Answer is {}", count);
    }

    fn part_2(&self, raw: &str) {
        println!("Day {} part 2", self.day());

        let answer = cheat(raw);

        println!("Answer is {}", answer);
    }
}


#[derive(Debug, Hash, PartialEq, Eq)]
struct State(Pos2I, usize);

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.1.cmp(&other.1).reverse())
    }
}

fn count_possible_after(
    grid_size: &Pos2U,
    start: &Pos2I,
    rocks: &HashSet<Pos2I>,
    steps: usize,
) -> usize {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut possible: HashSet<Pos2I> = HashSet::new();
    let mut explored: HashSet<Pos2I> = HashSet::new();

    queue.push(State(start.clone(), 0));

    while let Some(State(cur_pos, cur_step)) = queue.pop() {
        print!("step: {}\r", cur_step);
        if cur_step > steps {
            break;
        }
        let wrapped_pos = Pos2I(
            cur_pos.0 % grid_size.0 as isize,
            cur_pos.1 % grid_size.1 as isize,
        );
        if rocks.contains(&wrapped_pos) {
            continue;
        }
        if explored.contains(&cur_pos) {
            continue;
        }
        explored.insert(cur_pos.clone());
        if cur_step % 2 == steps % 2 {
            possible.insert(cur_pos.clone());
        }
        for dir in Dir4::all().iter() {
            queue.push(State(
                (cur_pos + dir.dir()).try_into().unwrap(),
                cur_step + 1,
            ));
        }
    }

    possible.len()
}


// All code after here from:
// https://gist.github.com/icub3d/70d8aced2636ee631b66cdb590185df7

// Standard point in the garden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

// We'll do some arithmatic on points to calculate next valid points.
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Find all neighbors of this point. We'll filter it later.
    fn neighbors(&self) -> Vec<Self> {
        vec![
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y - 1),
            Self::new(self.x, self.y + 1),
        ]
    }
}

struct Garden {
    rocks: HashSet<Point>,
    start: Point,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}
impl Garden {
    fn new(input: &str) -> Self {
        let max_y = input.lines().count() as i32;
        let max_x = input.lines().next().unwrap().len() as i32;
        let min_x = 0;
        let min_y = 0;

        // Go through the input and look for rocks and the start.
        let mut start = Point::new(0, 0);
        let mut rocks = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = Point::new(x as i32, y as i32);
                if c == 'S' {
                    start = p;
                } else if c == '#' {
                    rocks.insert(p);
                }
            }
        }

        Self {
            rocks,
            start,
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    // Find all valid neighbors of this point. Valid here means it's
    // in the garden and not a rock.
    fn neighbors(&self, p: &Point) -> Vec<Point> {
        p.neighbors()
            .into_iter()
            .filter(|p| {
                p.x >= self.min_x
                    && p.x < self.max_x
                    && p.y >= self.min_y
                    && p.y < self.max_y
                    && !self.rocks.contains(p)
            })
            .collect()
    }

    #[allow(dead_code)]
    fn naive(&self, steps: i32) -> HashSet<Point> {
        // We just use a helper function to recursively find all
        // points.
        let mut points = HashSet::new();
        self.naive_helper(&mut points, steps, &self.start);
        points
    }

    #[allow(dead_code)]
    fn naive_helper(&self, points: &mut HashSet<Point>, steps: i32, cur: &Point) {
        if steps == 0 {
            return;
        }
        for neighbor in self.neighbors(cur) {
            if steps == 1 {
                points.insert(neighbor);
            }
            self.naive_helper(points, steps - 1, &neighbor);
        }
    }

    fn alternating(&self, steps: i32) -> HashSet<Point> {
        let mut cur = HashSet::new();
        cur.insert(self.start);

        let mut next = HashSet::new();

        for _ in 0..steps {
            for p in cur.iter() {
                for d in self.neighbors(p) {
                    next.insert(d);
                }
            }
            std::mem::swap(&mut cur, &mut next);
            next.clear();
        }
        cur
    }

    #[allow(dead_code)]
    fn expand(&self) -> Self {
        let mut new_rocks = HashSet::new();
        for p in &self.rocks {
            new_rocks.insert(*p);
            new_rocks.insert(p.add(Point::new(0, self.max_y)));
            new_rocks.insert(p.add(Point::new(0, -self.max_y)));
            new_rocks.insert(p.add(Point::new(self.max_x, 0)));
            new_rocks.insert(p.add(Point::new(-self.max_x, 0)));
            new_rocks.insert(p.add(Point::new(self.max_x, self.max_y)));
            new_rocks.insert(p.add(Point::new(-self.max_x, self.max_y)));
            new_rocks.insert(p.add(Point::new(self.max_x, -self.max_y)));
            new_rocks.insert(p.add(Point::new(-self.max_x, -self.max_y)));
        }

        Self {
            rocks: new_rocks,
            start: self.start,
            min_x: self.min_x - self.max_x,
            min_y: self.min_y - self.max_y,
            max_x: self.max_x + self.max_x,
            max_y: self.max_y + self.max_y,
        }
    }

    fn calculate_distances(&self) -> HashMap<Point, i32> {
        let mut distances = HashMap::new();
        let mut frontier = VecDeque::new();
        frontier.push_back((self.start, 0));

        while let Some((p, dist)) = frontier.pop_front() {
            if distances.contains_key(&p) {
                continue;
            }

            distances.insert(p, dist);

            for neighbor in self.neighbors(&p) {
                frontier.push_back((neighbor, dist + 1));
            }
        }

        distances
    }
}

fn cheat(raw: &str) -> usize {
    // Build the garden.
    let garden = Garden::new(raw);
    
    // Expand the garden - I used this during the analysis of part 2.
    // let bigger_garden = garden.expand();
    // let bigger_points = bigger_garden.alternating(65 * 3 + 1);
    // bigger_garden.print(&bigger_points);

    // Calculate the distances from each point to the start.
    let distances = garden.calculate_distances();

    // In our final expanded garden, we'll have some odd and even
    // completely filled in blocks. We'll also have some edges of odds
    // we'll need to exclude and some edges of evens we'll need to
    // include. Start by getting a count of all those.
    let (odd, even, odd_edges, even_edges) = distances.iter().fold(
        (0_usize, 0_usize, 0_usize, 0_usize),
        |(odd, even, odd_edges, even_edges), (_, v)| {
            if *v % 2 == 1 && *v > 65 {
                (odd + 1, even, odd_edges + 1, even_edges)
            } else if *v % 2 == 1 {
                (odd + 1, even, odd_edges, even_edges)
            } else if *v % 2 == 0 && *v > 65 {
                (odd, even + 1, odd_edges, even_edges + 1)
            } else {
                (odd, even + 1, odd_edges, even_edges)
            }
        },
    );

    // Where do we get this value?
    // number of steps =  26501365
    // garden size = 131
    //   26501365 / 131 = 202300.496183 -- didn't work.
    //   26501365 % 131 = 65 -- it is a multiple, try ignoring center.
    //   (26501365 - 65) / 131 = 202300
    let count = 202300; // Magic number from analysis above.

    // The total odd and even are squares of count + 1 and count
    // respectively.
    let total_odd = odd * (count + 1) * (count + 1);
    let total_even = even * (count * count);

    // The odd edges is count + 1 and the even edges is count.
    let total_odd_edges = odd_edges * (count + 1);
    let total_even_edges = count * even_edges;

    let p2 = total_odd + total_even - total_odd_edges + total_even_edges;

    return p2;
}