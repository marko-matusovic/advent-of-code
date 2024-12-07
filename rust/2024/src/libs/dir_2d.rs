use super::pos_2i::Pos2I;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Dir4 {
    E,
    S,
    W,
    N,
}

impl From<usize> for Dir4 {
    fn from(value: usize) -> Self {
        match value % 360 {
            0 => Dir4::E,
            90 => Dir4::S,
            180 => Dir4::W,
            270 => Dir4::N,
            n => panic!(
                "Invalid value {}. Dir4 can only be build from multiples of 90.",
                n
            ),
        }
    }
}
impl Dir4 {
    pub fn deg(&self) -> usize {
        match self {
            Dir4::E => 0,
            Dir4::S => 90,
            Dir4::W => 180,
            Dir4::N => 270,
        }
    }
    pub fn dir(&self) -> Pos2I {
        match self {
            Dir4::E => Pos2I(1, 0),
            Dir4::S => Pos2I(0, 1),
            Dir4::W => Pos2I(-1, 0),
            Dir4::N => Pos2I(0, -1),
        }
    }
    pub fn is_horizontal(&self) -> bool {
        *self == Dir4::W || *self == Dir4::E
    }
    pub fn is_vertical(&self) -> bool {
        *self == Dir4::N || *self == Dir4::S
    }
    pub fn all() -> Vec<Self> {
        vec![Dir4::E, Dir4::S, Dir4::W, Dir4::N]
    }
    pub fn rotate(&self, deg: isize) -> Self {
        Self::from(self.deg() + (360 + deg % 360) as usize)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Dir8 {
    E,
    SE,
    S,
    SW,
    W,
    NW,
    N,
    NE,
}

impl From<usize> for Dir8 {
    fn from(value: usize) -> Self {
        match value % 360 {
            0 => Dir8::E,
            45 => Dir8::SE,
            90 => Dir8::S,
            135 => Dir8::SW,
            180 => Dir8::W,
            225 => Dir8::NW,
            270 => Dir8::N,
            315 => Dir8::NE,
            n => panic!(
                "Invalid value {}. Dir8 can only be build from multiples of 45.",
                n
            ),
        }
    }
}
impl Dir8 {
    pub fn deg(&self) -> usize {
        match self {
            Dir8::E => 0,
            Dir8::SE => 45,
            Dir8::S => 90,
            Dir8::SW => 135,
            Dir8::W => 180,
            Dir8::NW => 225,
            Dir8::N => 270,
            Dir8::NE => 315,
        }
    }
    pub fn dir(&self) -> Pos2I {
        match self {
            Dir8::E => Pos2I(1, 0),
            Dir8::SE => Pos2I(1, 1),
            Dir8::S => Pos2I(0, 1),
            Dir8::SW => Pos2I(-1, 1),
            Dir8::W => Pos2I(-1, 0),
            Dir8::NW => Pos2I(-1, -1),
            Dir8::N => Pos2I(0, -1),
            Dir8::NE => Pos2I(1, -1),
        }
    }
    pub fn is_horizontal(&self) -> bool {
        *self == Dir8::W || *self == Dir8::E
    }
    pub fn is_vertical(&self) -> bool {
        *self == Dir8::N || *self == Dir8::S
    }
    pub fn all() -> Vec<Self> {
        vec![
            Dir8::E,
            Dir8::SE,
            Dir8::S,
            Dir8::SW,
            Dir8::W,
            Dir8::NW,
            Dir8::N,
            Dir8::NE,
        ]
    }
    pub fn rotate(&self, deg: isize) -> Self {
        Self::from(self.deg() + (360 + deg % 360) as usize)
    }
}
