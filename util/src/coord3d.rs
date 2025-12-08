use crate::error::Errors;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UCoord3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ICoord3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Add<UCoord3D> for UCoord3D {
    type Output = UCoord3D;

    #[inline(always)]
    fn add(self, rhs: UCoord3D) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<UCoord3D> for UCoord3D {
    #[inline(always)]
    fn add_assign(&mut self, rhs: UCoord3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<UCoord3D> for UCoord3D {
    type Output = UCoord3D;

    #[inline(always)]
    fn sub(self, rhs: UCoord3D) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add<ICoord3D> for UCoord3D {
    type Output = ICoord3D;

    #[inline(always)]
    fn add(self, rhs: ICoord3D) -> Self::Output {
        Self::Output {
            x: self.x as isize + rhs.x,
            y: self.y as isize + rhs.y,
            z: self.z as isize + rhs.z,
        }
    }
}

impl Add<ICoord3D> for ICoord3D {
    type Output = ICoord3D;

    #[inline(always)]
    fn add(self, rhs: ICoord3D) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<ICoord3D> for ICoord3D {
    #[inline(always)]
    fn add_assign(&mut self, rhs: ICoord3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul<isize> for ICoord3D {
    type Output = ICoord3D;

    #[inline(always)]
    fn mul(self, rhs: isize) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Sub<ICoord3D> for ICoord3D {
    type Output = ICoord3D;

    #[inline(always)]
    fn sub(self, rhs: ICoord3D) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<ICoord3D> for ICoord3D {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: ICoord3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl TryFrom<ICoord3D> for UCoord3D {
    type Error = crate::error::Errors;

    #[inline(always)]
    fn try_from(val: ICoord3D) -> Result<Self, Self::Error> {
        match val {
            ICoord3D { x, y: _, z: _ } if x < 0 => Err(Errors::ConversionError),
            ICoord3D { x: _, y, z: _ } if y < 0 => Err(Errors::ConversionError),
            ICoord3D { x: _, y: _, z } if z < 0 => Err(Errors::ConversionError),
            ICoord3D { x, y, z } => Ok(Self {
                x: x as usize,
                y: y as usize,
                z: z as usize,
            }),
        }
    }
}

impl TryFrom<UCoord3D> for ICoord3D {
    type Error = crate::error::Errors;

    #[inline(always)]
    fn try_from(val: UCoord3D) -> Result<Self, Self::Error> {
        match val {
            UCoord3D { x, y: _, z: _ } if x > isize::MAX as usize => Err(Errors::ConversionError),
            UCoord3D { x: _, y, z: _ } if y > isize::MAX as usize => Err(Errors::ConversionError),
            UCoord3D { x: _, y: _, z } if z > isize::MAX as usize => Err(Errors::ConversionError),
            UCoord3D { x, y, z } => Ok(Self {
                x: x as isize,
                y: y as isize,
                z: z as isize,
            }),
        }
    }
}

pub trait TryAsUCoord3D {
    fn try_as_uucord3d(self) -> Result<UCoord3D, Errors>;
}

impl TryAsUCoord3D for UCoord3D {
    #[inline(always)]
    fn try_as_uucord3d(self) -> Result<UCoord3D, Errors> {
        Ok(self)
    }
}

impl TryAsUCoord3D for ICoord3D {
    #[inline(always)]
    fn try_as_uucord3d(self) -> Result<UCoord3D, Errors> {
        self.try_into()
    }
}

pub trait TryAsICoord3D {
    fn try_as_iucord3d(self) -> Result<ICoord3D, Errors>;
}

impl TryAsICoord3D for ICoord3D {
    #[inline(always)]
    fn try_as_iucord3d(self) -> Result<ICoord3D, Errors> {
        Ok(self)
    }
}

impl TryAsICoord3D for UCoord3D {
    #[inline(always)]
    fn try_as_iucord3d(self) -> Result<ICoord3D, Errors> {
        self.try_into()
    }
}

impl ICoord3D {
    #[inline(always)]
    pub fn magnitude(&self) -> f64 {
        (self.x as f64 * self.x as f64
            + self.y as f64 * self.y as f64
            + self.z as f64 * self.z as f64)
            .sqrt()
    }
}
