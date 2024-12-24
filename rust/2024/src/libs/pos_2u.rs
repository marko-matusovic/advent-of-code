use std::{
    iter::repeat,
    ops::{Add, Sub},
};

use itertools::Itertools;

use super::pos_2i::Pos2I;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos2U(pub usize, pub usize);

impl ToString for Pos2U {
    fn to_string(&self) -> String {
        format!("Pos2U({},{})", self.0, self.1)
    }
}

impl FromIterator<usize> for Pos2U {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let (a, b) = iter
            .into_iter()
            .chain(repeat(0))
            .take(2)
            .collect_tuple()
            .unwrap();
        Self(a, b)
    }
}

impl Into<Pos2I> for Pos2U {
    fn into(self) -> Pos2I {
        Pos2I(self.0 as isize, self.1 as isize)
    }
}

impl Add<Pos2U> for Pos2U {
    type Output = Pos2U;

    fn add(self, other: Self) -> Self::Output {
        Pos2U(self.0 + other.0, self.1 + other.1)
    }
}
impl Add<Pos2I> for Pos2U {
    type Output = Pos2I;

    fn add(self, other: Pos2I) -> Self::Output {
        Pos2I(self.0 as isize + other.0, self.1 as isize + other.1)
    }
}
impl Sub for Pos2U {
    type Output = Pos2I;

    fn sub(self, other: Self) -> Self::Output {
        Pos2I(
            self.0 as isize - other.0 as isize,
            self.1 as isize - other.1 as isize,
        )
    }
}
impl Pos2U {
    pub fn add_unwrap(self, other: Pos2I) -> Pos2U {
        self.add(other).try_into().unwrap()
    }
    pub fn scale(self, sc: usize) -> Self {
        Pos2U(self.0 * sc, self.1 * sc)
    }
    pub fn dominates(&self, other: Self) -> bool {
        return other.0 <= self.0 && other.1 <= self.1;
    }
    pub fn dist_n1(&self, other: Self) -> usize {
        ((self.0 as isize - other.0 as isize).abs() + (self.1 as isize - other.1 as isize).abs())
            as usize
    }
    pub fn dist_n2_sq(&self, other: Self) -> usize {
        let a = self.0 as isize - other.0 as isize;
        let b = self.1 as isize - other.1 as isize;
        (a * a + b * b) as usize
    }
    pub fn dist_n2(&self, other: Self) -> f64 {
        (self.dist_n2_sq(other) as f64).sqrt()
    }
}
