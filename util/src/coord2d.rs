use crate::error::Errors;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UCoord2D {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ICoord2D {
    pub x: isize,
    pub y: isize,
}

impl Add<UCoord2D> for UCoord2D {
    type Output = UCoord2D;

    #[inline(always)]
    fn add(self, rhs: UCoord2D) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<UCoord2D> for UCoord2D {
    type Output = UCoord2D;

    #[inline(always)]
    fn sub(self, rhs: UCoord2D) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<ICoord2D> for UCoord2D {
    type Output = ICoord2D;

    #[inline(always)]
    fn add(self, rhs: ICoord2D) -> Self::Output {
        Self::Output {
            x: self.x as isize + rhs.x,
            y: self.y as isize + rhs.y,
        }
    }
}

impl Add<ICoord2D> for ICoord2D {
    type Output = ICoord2D;

    #[inline(always)]
    fn add(self, rhs: ICoord2D) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<ICoord2D> for ICoord2D {
    #[inline(always)]
    fn add_assign(&mut self, rhs: ICoord2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<isize> for ICoord2D {
    type Output = ICoord2D;

    #[inline(always)]
    fn mul(self, rhs: isize) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Sub<ICoord2D> for ICoord2D {
    type Output = ICoord2D;

    #[inline(always)]
    fn sub(self, rhs: ICoord2D) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<ICoord2D> for ICoord2D {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: ICoord2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl TryFrom<ICoord2D> for UCoord2D {
    type Error = crate::error::Errors;

    #[inline(always)]
    fn try_from(val: ICoord2D) -> Result<Self, Self::Error> {
        match val {
            ICoord2D { x, y: _ } if x < 0 => Err(Errors::ConversionError),
            ICoord2D { x: _, y } if y < 0 => Err(Errors::ConversionError),
            ICoord2D { x, y } => Ok(Self {
                x: x as usize,
                y: y as usize,
            }),
        }
    }
}

impl TryFrom<UCoord2D> for ICoord2D {
    type Error = crate::error::Errors;

    #[inline(always)]
    fn try_from(val: UCoord2D) -> Result<Self, Self::Error> {
        match val {
            UCoord2D { x, y: _ } if x > isize::MAX as usize => Err(Errors::ConversionError),
            UCoord2D { x: _, y } if y > isize::MAX as usize => Err(Errors::ConversionError),
            UCoord2D { x, y } => Ok(Self {
                x: x as isize,
                y: y as isize,
            }),
        }
    }
}

pub trait TryAsUCoord2D {
    fn try_as_uucord2d(self) -> Result<UCoord2D, Errors>;
}

impl TryAsUCoord2D for UCoord2D {
    #[inline(always)]
    fn try_as_uucord2d(self) -> Result<UCoord2D, Errors> {
        Ok(self)
    }
}

impl TryAsUCoord2D for ICoord2D {
    #[inline(always)]
    fn try_as_uucord2d(self) -> Result<UCoord2D, Errors> {
        self.try_into()
    }
}

pub trait TryAsICoord2D {
    fn try_as_iucord2d(self) -> Result<ICoord2D, Errors>;
}

impl TryAsICoord2D for ICoord2D {
    #[inline(always)]
    fn try_as_iucord2d(self) -> Result<ICoord2D, Errors> {
        Ok(self)
    }
}

impl TryAsICoord2D for UCoord2D {
    #[inline(always)]
    fn try_as_iucord2d(self) -> Result<ICoord2D, Errors> {
        self.try_into()
    }
}
