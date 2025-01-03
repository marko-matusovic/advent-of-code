use std::{
    iter::repeat,
    ops::{Add, Sub},
};

use itertools::Itertools;

use super::pos_2u::Pos2U;
use std::convert::TryInto;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos2I(pub isize, pub isize);

impl ToString for Pos2I {
    fn to_string(&self) -> String {
        format!("Pos2I({},{})", self.0, self.1)
    }
}

impl FromIterator<isize> for Pos2I {
    fn from_iter<T: IntoIterator<Item = isize>>(iter: T) -> Self {
        let (a, b) = iter
            .into_iter()
            .chain(repeat(0))
            .take(2)
            .collect_tuple()
            .unwrap();
        Self(a, b)
    }
}

impl TryInto<Pos2U> for Pos2I {
    type Error = String;

    fn try_into(self) -> Result<Pos2U, Self::Error> {
        if self.0.is_negative() || self.1.is_negative() {
            Err("Cannot convert into Pos2U, contains negative coordinates!".to_owned())
        } else {
            Ok(Pos2U(self.0 as usize, self.1 as usize))
        }
    }
}

impl Add for Pos2I {
    type Output = Pos2I;

    fn add(self, other: Self) -> Self::Output {
        Pos2I(self.0 + other.0, self.1 + other.1)
    }
}
impl Sub for Pos2I {
    type Output = Pos2I;

    fn sub(self, other: Self) -> Self::Output {
        Pos2I(self.0 - other.0, self.1 - other.1)
    }
}
impl Pos2I {
    pub fn scale(self, sc: isize) -> Self {
        Pos2I(self.0 * sc, self.1 * sc)
    }
    pub fn dominates(&self, other: Self) -> bool {
        return other.0 <= self.0 && other.1 <= self.1;
    }
    pub fn dist_n1(&self, other: Self) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
    pub fn dist_n2_sq(&self, other: Self) -> isize {
        let a = self.0 - other.0;
        let b = self.1 - other.1;
        a * a + b * b
    }
    pub fn dist_n2(&self, other: Self) -> f64 {
        (self.dist_n2_sq(other) as f64).sqrt()
    }
    pub fn wrap(&self, other: Self) -> Self {
        Pos2I(
            ((self.0 % other.0) + other.0) % other.0,
            ((self.1 % other.1) + other.1) % other.1,
        )
    }
}
