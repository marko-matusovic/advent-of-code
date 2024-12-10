use super::Pos2U;

#[derive(Debug)]
pub struct Grid2<I>(pub Vec<Vec<I>>);

impl<I: Copy> From<Vec<Vec<I>>> for Grid2<I> {
    fn from(value: Vec<Vec<I>>) -> Self {
        Grid2(value)
    }
}

impl<I> Grid2<I> {
    pub fn get(&self, at: &Pos2U) -> Option<&I> {
        if let Some(row) = self.0.get(at.1) {
            return row.get(at.0);
        }
        None
    }
}

impl<I: Clone> Clone for Grid2<I> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<I: Clone> Grid2<I> {
    pub fn rotate_cw(&self) -> Grid2<I> {
        let mut new_grid: Vec<Vec<I>> = Vec::new();

        for y in 0..self.0.len() {
            new_grid.push(Vec::new());
            for x in 0..self.0[y].len() {
                new_grid[y].push(self.0[self.0[y].len() - x - 1][y].clone());
            }
        }

        Grid2(new_grid)
    }

    pub fn rotate_ccw(&self) -> Grid2<I> {
        let mut new_grid: Vec<Vec<I>> = Vec::new();

        for y in 0..self.0.len() {
            new_grid.push(Vec::new());
            for x in 0..self.0[y].len() {
                new_grid[y].push(self.0[x][self.0.len() - y - 1].clone());
            }
        }

        Grid2(new_grid)
    }

    pub fn transpose(&self) -> Grid2<I> {
        let mut new_grid: Vec<Vec<I>> = Vec::new();

        for y in 0..self.0.len() {
            new_grid.push(Vec::new());
            for x in 0..self.0[y].len() {
                new_grid[y].push(self.0[x][y].clone());
            }
        }

        Grid2(new_grid)
    }
}

impl<I: ToString> Grid2<I> {
    pub fn print(&self) {
        for line in self.0.iter() {
            for ch in line {
                print!("{}", ch.to_string());
            }
            println!();
        }
    }
}
