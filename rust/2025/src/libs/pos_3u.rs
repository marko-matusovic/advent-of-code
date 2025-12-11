use std::{
    iter::repeat,
    ops::{Add, Sub},
};

use itertools::Itertools;

use super::pos_3i::Pos3I;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
pub struct Pos3U(pub usize, pub usize, pub usize);

impl ToString for Pos3U {
    fn to_string(&self) -> String {
        format!("Pos3U({},{},{})", self.0, self.1, self.2)
    }
}

impl FromIterator<usize> for Pos3U {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let (a, b, c) = iter
            .into_iter()
            .chain(repeat(0))
            .take(3)
            .collect_tuple()
            .unwrap();
        Self(a, b, c)
    }
}

impl Into<Pos3I> for Pos3U {
    fn into(self) -> Pos3I {
        Pos3I(self.0 as isize, self.1 as isize, self.2 as isize)
    }
}

impl Add<Pos3U> for Pos3U {
    type Output = Pos3U;

    fn add(self, other: Self) -> Self::Output {
        Pos3U(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl Add<Pos3I> for Pos3U {
    type Output = Pos3I;

    fn add(self, other: Pos3I) -> Self::Output {
        Pos3I(
            self.0 as isize + other.0,
            self.1 as isize + other.1,
            self.2 as isize + other.2,
        )
    }
}
impl Sub for Pos3U {
    type Output = Pos3I;

    fn sub(self, other: Self) -> Self::Output {
        Pos3I(
            self.0 as isize - other.0 as isize,
            self.1 as isize - other.1 as isize,
            self.2 as isize - other.2 as isize,
        )
    }
}
impl Pos3U {
    pub fn scale(self, sc: usize) -> Self {
        Pos3U(self.0 * sc, self.1 * sc, self.2 * sc)
    }
    pub fn dominates(&self, other: &Self) -> bool {
        other.0 <= self.0 && other.1 <= self.1 && other.2 <= self.2
    }
    pub fn dist_x(&self, other: &Self) -> usize {
        (self.0 as isize - other.0 as isize).abs() as usize
    }
    pub fn dist_y(&self, other: &Self) -> usize {
        (self.1 as isize - other.1 as isize).abs() as usize
    }
    pub fn dist_z(&self, other: &Self) -> usize {
        (self.2 as isize - other.2 as isize).abs() as usize
    }
    pub fn dist_n1(&self, other: &Self) -> usize {
        ((self.0 as isize - other.0 as isize).abs()
            + (self.1 as isize - other.1 as isize).abs()
            + (self.2 as isize - other.2 as isize).abs()) as usize
    }
    pub fn dist_n2_sq(&self, other: &Self) -> usize {
        let a = self.0 as isize - other.0 as isize;
        let b = self.1 as isize - other.1 as isize;
        let c = self.2 as isize - other.2 as isize;
        (a * a + b * b + c * c) as usize
    }
    pub fn dist_n2(&self, other: &Self) -> f64 {
        (self.dist_n2_sq(other) as f64).sqrt()
    }
}
