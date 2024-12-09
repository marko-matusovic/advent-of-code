pub fn rotate_cw<I: Clone>(grid: Vec<Vec<I>>) -> Vec<Vec<I>> {
    let mut new_grid: Vec<Vec<I>> = Vec::new();

    for y in 0..grid.len() {
        new_grid.push(Vec::new());
        for x in 0..grid[y].len() {
            new_grid[y].push(grid[grid[y].len() - x - 1][y].clone());
        }
    }

    new_grid
}

pub fn rotate_ccw<I: Clone>(grid: Vec<Vec<I>>) -> Vec<Vec<I>> {
    let mut new_grid: Vec<Vec<I>> = Vec::new();

    for y in 0..grid.len() {
        new_grid.push(Vec::new());
        for x in 0..grid[y].len() {
            new_grid[y].push(grid[x][grid.len() - y - 1].clone());
        }
    }

    new_grid
}

pub fn transpose<I: Clone>(grid: Vec<Vec<I>>) -> Vec<Vec<I>> {
    let mut new_grid: Vec<Vec<I>> = Vec::new();

    for y in 0..grid.len() {
        new_grid.push(Vec::new());
        for x in 0..grid[y].len() {
            new_grid[y].push(grid[x][y].clone());
        }
    }

    new_grid
}
