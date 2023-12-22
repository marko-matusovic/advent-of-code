use std::ops::{Add, Sub};

use super::pos_3u::Pos3U;
use std::convert::TryInto;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos3I(pub isize, pub isize, pub isize);

impl TryInto<Pos3U> for Pos3I {
    type Error = String;

    fn try_into(self) -> Result<Pos3U, Self::Error> {
        if self.0.is_negative() || self.1.is_negative() || self.2.is_negative() {
            Err("Cannot convert into Pos3U, contains negative coordinates!".to_owned())
        } else {
            Ok(Pos3U(self.0 as usize, self.1 as usize, self.2 as usize))
        }
    }
}

impl Add for Pos3I {
    type Output = Pos3I;

    fn add(self, other: Self) -> Self::Output {
        Pos3I(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl Sub for Pos3I {
    type Output = Pos3I;

    fn sub(self, other: Self) -> Self::Output {
        Pos3I(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl Pos3I {
    pub fn scale(self, sc: isize) -> Self {
        Pos3I(self.0 * sc, self.1 * sc, self.2 * sc)
    }
    pub fn dominates(&self, other: Self) -> bool {
        return other.0 <= self.0 && other.1 <= self.1 && other.2 <= self.2;
    }
    pub fn dist_n1(&self, other: Self) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
    }
    pub fn dist_n2_sq(&self, other: Self) -> isize {
        let a = self.0 - other.0;
        let b = self.1 - other.1;
        let c = self.2 - other.2;
        a * a + b * b + c * c
    }
    pub fn dist_n2(&self, other: Self) -> f64 {
        (self.dist_n2_sq(other) as f64).sqrt()
    }
}
