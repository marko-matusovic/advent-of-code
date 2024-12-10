use super::Pos3I;

type AxisRotation = (usize, usize, usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Dir6 {
    F, // X
    B,
    R, // Y
    L,
    U, // Z
    D,
}

impl ToString for Dir6 {
    fn to_string(&self) -> String {
        format!("Dir6::{}", match self {
            Dir6::F => "F",
            Dir6::B => "B",
            Dir6::R => "R",
            Dir6::L => "L",
            Dir6::U => "U",
            Dir6::D => "D",
        })
    }
}

impl Dir6 {
    pub fn dir(&self) -> Pos3I {
        match self {
            Dir6::F => Pos3I(1, 0, 0),
            Dir6::B => Pos3I(-1, 0, 0),
            Dir6::R => Pos3I(0, 1, 0),
            Dir6::L => Pos3I(0, -1, 0),
            Dir6::U => Pos3I(0, 0, 1),
            Dir6::D => Pos3I(0, 0, -1),
        }
    }
    pub fn deg_x(&self) -> Option<usize> {
        match *self {
            Dir6::U => Some(0),
            Dir6::R => Some(90),
            Dir6::D => Some(180),
            Dir6::L => Some(270),
            _ => None,
        }
    }
    pub fn from_x(d: usize) -> Self {
        match d % 360 {
            0 => Dir6::U,
            90 => Dir6::R,
            180 => Dir6::D,
            270 => Dir6::L,
            _ => panic!(),
        }
    }
    pub fn rotate_x(&self, x: isize) -> Self {
        if let Some(dg) = self.deg_x() {
            Self::from_x((360 + x % 360) as usize + dg)
        } else {
            self.clone()
        }
    }
    pub fn deg_y(&self) -> Option<usize> {
        match *self {
            Dir6::U => Some(0),
            Dir6::B => Some(90),
            Dir6::D => Some(180),
            Dir6::F => Some(270),
            _ => None,
        }
    }
    pub fn from_y(d: usize) -> Self {
        match d % 360 {
            0 => Dir6::U,
            90 => Dir6::B,
            180 => Dir6::D,
            270 => Dir6::F,
            _ => panic!(),
        }
    }
    pub fn rotate_y(&self, y: usize) -> Self {
        if let Some(dg) = self.deg_y() {
            Self::from_y((360 + y % 360) as usize + dg)
        } else {
            self.clone()
        }
    }
    pub fn deg_z(&self) -> Option<usize> {
        match *self {
            Dir6::F => Some(0),
            Dir6::L => Some(90),
            Dir6::B => Some(180),
            Dir6::R => Some(270),
            _ => None,
        }
    }
    pub fn from_z(d: usize) -> Self {
        match d % 360 {
            0 => Dir6::F,
            90 => Dir6::L,
            180 => Dir6::B,
            270 => Dir6::R,
            _ => panic!(),
        }
    }
    pub fn rotate_z(&self, z: usize) -> Self {
        if let Some(dg) = self.deg_z() {
            Self::from_z((360 + z % 360) as usize + dg)
        } else {
            self.clone()
        }
    }
    pub fn all() -> Vec<Self> {
        vec![Dir6::U, Dir6::D, Dir6::F, Dir6::B, Dir6::R, Dir6::L]
    }
}
