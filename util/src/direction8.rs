use crate::coord2d::ICoord2D;
use crate::direction8::Direction8::{
    East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Direction8 {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction8 {
    #[inline(always)]
    pub fn to_offset(self) -> ICoord2D {
        match self {
            North => ICoord2D { x: 0, y: -1 },
            NorthEast => ICoord2D { x: 1, y: -1 },
            East => ICoord2D { x: 1, y: 0 },
            SouthEast => ICoord2D { x: 1, y: 1 },
            South => ICoord2D { x: 0, y: 1 },
            SouthWest => ICoord2D { x: -1, y: 1 },
            West => ICoord2D { x: -1, y: 0 },
            NorthWest => ICoord2D { x: -1, y: -1 },
        }
    }

    #[inline(always)]
    pub fn rotate_left(self) -> Direction8 {
        match self {
            North => NorthWest,
            NorthEast => North,
            East => NorthEast,
            SouthEast => East,
            South => SouthEast,
            SouthWest => South,
            West => SouthWest,
            NorthWest => West,
        }
    }

    #[inline(always)]
    pub fn rotate_right(self) -> Direction8 {
        match self {
            North => NorthEast,
            NorthEast => East,
            East => SouthEast,
            SouthEast => South,
            South => SouthWest,
            SouthWest => West,
            West => NorthWest,
            NorthWest => North,
        }
    }

    #[inline(always)]
    pub const fn directions() -> [Direction8; 8] {
        [
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ]
    }
}
