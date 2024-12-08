use crate::error::Errors;
use std::ops::Sub;
use std::ops::{Add, SubAssign};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct UCoord2D {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct ICoord2D {
    pub x: isize,
    pub y: isize,
}

impl Add<UCoord2D> for UCoord2D {
    type Output = UCoord2D;
    fn add(self, rhs: UCoord2D) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<UCoord2D> for UCoord2D {
    type Output = UCoord2D;
    fn sub(self, rhs: UCoord2D) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<ICoord2D> for UCoord2D {
    type Output = ICoord2D;
    fn add(self, rhs: ICoord2D) -> Self::Output {
        Self::Output {
            x: self.x as isize + rhs.x,
            y: self.y as isize + rhs.y,
        }
    }
}

impl Add<ICoord2D> for ICoord2D {
    type Output = ICoord2D;
    fn add(self, rhs: ICoord2D) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<ICoord2D> for ICoord2D {
    type Output = ICoord2D;
    fn sub(self, rhs: ICoord2D) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<ICoord2D> for ICoord2D {
    fn sub_assign(&mut self, rhs: ICoord2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl TryInto<UCoord2D> for ICoord2D {
    type Error = crate::error::Errors;

    fn try_into(self) -> Result<UCoord2D, Self::Error> {
        match self {
            ICoord2D { x, y: _ } if x < 0 => Err(Errors::ConversionError),
            ICoord2D { x: _, y } if y < 0 => Err(Errors::ConversionError),
            ICoord2D { x, y } => Ok(UCoord2D {
                x: x as usize,
                y: y as usize,
            }),
        }
    }
}

impl TryInto<ICoord2D> for UCoord2D {
    type Error = crate::error::Errors;

    fn try_into(self) -> Result<ICoord2D, Self::Error> {
        match self {
            UCoord2D { x, y: _ } if x > isize::MAX as usize => Err(Errors::ConversionError),
            UCoord2D { x: _, y } if y > isize::MAX as usize => Err(Errors::ConversionError),
            UCoord2D { x, y } => Ok(ICoord2D {
                x: x as isize,
                y: y as isize,
            }),
        }
    }
}
