use crate::coord2d::ICoord2D;
use crate::direction::Direction::{East, North, South, West};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    #[inline(always)]
    pub fn to_offset(self) -> ICoord2D {
        match self {
            North => ICoord2D { x: 0, y: -1 },
            East => ICoord2D { x: 1, y: 0 },
            South => ICoord2D { x: 0, y: 1 },
            West => ICoord2D { x: -1, y: 0 },
        }
    }

    #[inline(always)]
    pub fn rotate_left(self) -> Direction {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    #[inline(always)]
    pub fn rotate_right(self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    #[inline(always)]
    pub const fn directions() -> [Direction; 4] {
        [North, East, South, West]
    }
}
