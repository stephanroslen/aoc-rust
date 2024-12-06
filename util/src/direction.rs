use crate::coord2d::ICoord2D;
use crate::direction::Direction::{East, North, South, West};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn to_offset(self) -> ICoord2D {
        match self {
            North => ICoord2D { x: 0, y: -1 },
            East => ICoord2D { x: 1, y: 0 },
            South => ICoord2D { x: 0, y: 1 },
            West => ICoord2D { x: -1, y: 0 },
        }
    }

    pub fn rotate_right(self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}
