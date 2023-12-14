use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn day() -> u8 {
    24
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Grid(Vec<Vec<Vec<char>>>);

impl Grid {
    fn step(&self) -> Self {
        let mut next = Vec::new();
        for _ in 0..self.0.len() {
            next.push(Vec::new());
            for _ in 0..self.0[0].len() {
                next.last_mut().unwrap().push(Vec::new());
            }
        }

        let h = next.len();
        let w = next[0].len();

        for (r, row) in self.0.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                for ch in col {
                    match *ch {
                        '^' => next[(h + r - 1) % h][c].push('^'),
                        'v' => next[(h + r + 1) % h][c].push('v'),
                        '<' => next[r][(w + c - 1) % w].push('<'),
                        '>' => next[r][(w + c + 1) % w].push('>'),
                        '.' => (), // do nothing
                        _ => panic!(),
                    }
                }
            }
        }

        Grid(next)
    }

    fn print(&self) {
        for row in &self.0 {
            for col in row {
                print!(
                    "{}",
                    if col.len() == 0 {
                        '.'
                    } else if col.len() == 1 {
                        col[0]
                    } else if col.len() < 10 {
                        col.len().to_string().chars().nth(0).unwrap()
                    } else {
                        panic!()
                    }
                )
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Input {
    lines: Vec<String>,
    grid: Grid,
}

pub fn parse_input(raw: &str) -> Input {
    let lines: Vec<String> = raw.split("\n").map(|s| s.to_owned()).collect();
    println!("Day {} parsing {} lines", day(), lines.len());

    let grid = Grid(
        lines
            .iter()
            .skip(1)
            .take(lines.len() - 2)
            .map(|l| {
                l.chars()
                    .skip(1)
                    .take(l.len() - 2)
                    .map(|ch| vec![ch])
                    .collect()
            })
            .collect(),
    );

    Input { lines, grid }
}

type Pos = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State(Pos, Pos, usize);
// State(0: cur, 1: goal, 2: age)

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BFS
        // self.2.cmp(&other.2).reverse()

        // A*
        let dist = |a: usize, b: usize| (a as isize - b as isize).abs() as usize;
        let score = |st: &State| st.2 + dist(st.0.0, st.1.0) + dist(st.0.1, st.1.1);
        score(self).cmp(&score(other)).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn wait_for(grid: &Grid, pos: &Pos) -> (usize, Grid) {
    let mut cur = grid.clone();
    let mut age = 0;
    while !cur.0[pos.1][pos.0].is_empty() {
        cur = cur.step();
        age += 1;
    }
    (age, cur)
}

fn find_path(grid: &Grid, start: Pos, goal: Pos) -> Option<(usize, Grid)> {
    let mut grid_cache: HashMap<usize, Grid> = HashMap::new();
    grid_cache.insert(0, grid.clone());

    let w = grid.0[0].len() as isize;
    let h = grid.0.len() as isize;

    let mut states_set: HashSet<State> = HashSet::new();
    let mut states_queue: BinaryHeap<State> = BinaryHeap::new();
    states_set.insert(State(start, goal, 0));
    states_queue.push(State(start, goal, 0));

    while let Some(State(cur, _, age)) = states_queue.pop() {
        // println!("age: {:3}, pos: ({},{})", age, cur.0, cur.1);
        if cur == goal {
            return Some((age, grid_cache.get(&age).unwrap().to_owned()));
        }
        if !grid_cache.contains_key(&(age + 1)) {
            grid_cache.insert(age + 1, grid_cache.get(&age).unwrap().step());
        }
        let next_grid = grid_cache.get(&(age + 1)).unwrap();

        // println!("Current");
        // grid_cache.get(&age).unwrap().print();
        // println!("Next");
        // next_grid.print();

        for dir in vec![(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next_pos = (cur.0 as isize + dir.0, cur.1 as isize + dir.1);
            // println!("Next Pos: ({},{})", next_pos.0, next_pos.1);
            if 0 <= next_pos.0 && 0 <= next_pos.1 && next_pos.0 < w && next_pos.1 < h {
                let next_pos = (next_pos.0 as usize, next_pos.1 as usize);
                // dbg!(&next_grid.0[next_pos.1 as usize][next_pos.0 as usize]);
                let next_state = State(next_pos, goal, age + 1);
                if !states_set.contains(&next_state)
                    && next_grid.0[next_pos.1][next_pos.0].is_empty()
                {
                    // println!("Adding");
                    states_queue.push(next_state.clone());
                    states_set.insert(next_state);
                }
                // } else {
                // println!("Out of bounds");
            }
        }
    }
    None
}

fn solve(grid: &Grid, start: Pos, goal: Pos) -> (usize, Grid) {
    let mut grid = grid.step();
    let mut delay = 1;
    loop {
        let (wait, g1) = wait_for(&grid, &start);
        if let Some((walk, g2)) = find_path(&g1, start, goal) {
            return (delay + wait + walk + 1, g2.step());
        }
        delay += wait + 1;
        grid = g1.step();
    }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let start: Pos = (0, 0);
    let goal: Pos = (input.grid.0[0].len() - 1, input.grid.0.len() - 1);
    
    let (time, _) = solve(&input.grid, start, goal);

    println!("Answer is {}", time);
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());
    
    let start: Pos = (0, 0);
    let goal: Pos = (input.grid.0[0].len() - 1, input.grid.0.len() - 1);
    
    let (time1, grid) = solve(&input.grid, start, goal);
    let (time2, grid) = solve(&grid, goal, start);
    let (time3, _) = solve(&grid, start, goal);

    println!("Answer is {}", time1 + time2 + time3);
}
