use itertools::Itertools;

use super::{Pos2I, Pos2U};

#[derive(Debug)]
pub struct Grid2<I>(pub Vec<Vec<I>>);

impl<I: Copy> From<Vec<Vec<I>>> for Grid2<I> {
    fn from(value: Vec<Vec<I>>) -> Self {
        Grid2(value)
    }
}

impl From<&Vec<String>> for Grid2<char> {
    fn from(lines: &Vec<String>) -> Self {
        Grid2(lines.iter().map(|l| l.chars().collect_vec()).collect_vec())
    }
}

impl<I> Grid2<I> {
    pub fn get_at_posu(&self, at: &Pos2U) -> Option<&I> {
        self.get(at.0, at.1)
    }

    pub fn get_at_posi(&self, at: &Pos2I) -> Option<&I> {
        if at.0 < 0 || at.1 < 0 {
            return None;
        }
        self.get(at.0 as usize, at.1 as usize)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&I> {
        if let Some(row) = self.0.get(y) {
            return row.get(x);
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
    pub fn fill(element: &I, x: usize, y: usize) -> Grid2<I> {
        Grid2(vec![vec![element.clone(); x]; y])
    }

    pub fn x(&self) -> usize {
        self.0[0].len()
    }

    pub fn y(&self) -> usize {
        self.0.len()
    }

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
