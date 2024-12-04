use std::ops::Add;
use std::ops::Sub;

#[derive(Copy, Clone, Debug)]
pub struct UCoord2D {
    pub x: usize,
    pub y: usize,
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
