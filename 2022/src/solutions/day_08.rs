pub fn day() -> u8 {
    8
}

#[derive(Debug)]
pub struct Input {
    grid: Vec<Vec<u8>>,
}

pub fn parse_input(raw: &str) -> Input {
    let grid = raw
        .split("\n")
        .map(|l| l.chars().map(|ch| (ch as u8) - ('0' as u8)).collect())
        .collect();
    println!("Day {} parsing", day());
    Input { grid }
}

pub fn part_1(input: &Input) {
    println!("Day {} part 1", day());

    let grid = input.grid.clone();

    let total = (0..grid.len())
        .map(|y| {
            (0..grid[y].len())
                .map(|x| visible(&grid, x, y))
                .collect::<Vec<bool>>()
        })
        .flatten()
        .filter(|&v| v)
        .count();

    println!("Answer is {}", total);
}

fn visible(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    if x == 0 || x == grid.len() - 1 || y == 0 || y == grid[0].len() - 1 {
        return true;
    }

    let tree = grid[y][x];

    let mut res = true;

    for i in 0..x {
        if grid[y][i] >= tree {
            res = false;
            break;
        }
    }

    if res == true {
        return true;
    }
    res = true;
    for i in (x + 1)..grid[y].len() {
        if grid[y][i] >= tree {
            res = false;
            break;
        }
    }

    if res == true {
        return true;
    }
    res = true;
    for j in 0..y {
        if grid[j][x] >= tree {
            res = false;
            break;
        }
    }

    if res == true {
        return true;
    }
    res = true;
    for j in y + 1..grid.len() {
        if grid[j][x] >= tree {
            res = false;
            break;
        }
    }

    return res;
}

pub fn part_2(input: &Input) {
    println!("Day {} part 2", day());

    let grid = input.grid.clone();

    let total = (0..grid.len())
        .map(|y| {
            (0..grid[y].len())
                .map(|x| visibility_score(&grid, x, y))
                .collect::<Vec<u64>>()
        })
        .flatten()
        .max()
        .unwrap();

    println!("Answer is {}", total);
}


fn visibility_score(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> u64 {
    if x == 0 || x == grid.len() - 1 || y == 0 || y == grid[0].len() - 1 {
        return 0;
    }

    let tree = grid[y][x];


    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;

    for i in (0..x).rev() {
        left += 1;
        if grid[y][i] >= tree {
            break;
        }
    }

    for i in (x + 1)..grid[y].len() {
        right += 1;
        if grid[y][i] >= tree {
            break;
        }
    }

    for j in (0..y).rev() {
        up += 1;
        if grid[j][x] >= tree {
            break;
        }
    }

    for j in y + 1..grid.len() {
        down += 1;
        if grid[j][x] >= tree {
            break;
        }
    }

    return up * down * left * right;
}