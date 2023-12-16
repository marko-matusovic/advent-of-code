use std::ops::{Add, Sub};

use super::pos_2i::Pos2I;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos2U(pub usize, pub usize);

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
    type Output = Pos2U;

    fn sub(self, other: Self) -> Self::Output {
        Pos2U(self.0 - other.0, self.1 - other.1)
    }
}
impl Pos2U {
    fn scale(self, sc: usize) -> Self {
        Pos2U(self.0 * sc, self.1 * sc)
    }
    fn dominates(&self, other: Self) -> bool {
        return other.0 <= self.0 && other.1 <= self.1;
    }
    fn dist_n1(&self, other: Self) -> usize {
        ((self.0 as isize - other.0 as isize).abs() + (self.1 as isize - other.1 as isize).abs())
            as usize
    }
    fn dist_n2_sq(&self, other: Self) -> usize {
        let a = self.0 as isize - other.0 as isize;
        let b = self.0 as isize - other.0 as isize;
        (a * a + b * b) as usize
    }
    fn dist_n2(&self, other: Self) -> f64 {
        (self.dist_n2_sq(other) as f64).sqrt()
    }
}
