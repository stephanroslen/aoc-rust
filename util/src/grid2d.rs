use crate::coord2d::ICoord2D;
pub use crate::coord2d::TryAsUCoord2D;
pub use crate::coord2d::UCoord2D;
pub use crate::error::Errors;

#[derive(Clone, Debug)]
pub struct UGrid2D<T> {
    dim: UCoord2D,
    data: Vec<T>,
}

impl<T: Default + Clone> UGrid2D<T> {
    #[inline(always)]
    pub fn from_default(dim: UCoord2D) -> Self {
        Self::generate(dim, |_| Ok(T::default())).unwrap()
    }
}

impl<T: Clone> UGrid2D<T> {
    #[inline(always)]
    fn internal_index(&self, coord: impl TryAsUCoord2D) -> Result<usize, Errors> {
        let coord = coord.try_as_uucord2d()?;
        match coord {
            UCoord2D { x, y: _ } if x >= self.dim.x => Err(Errors::DimError(format!(
                "x ({}) equal or above dimensional bound ({})",
                x, self.dim.x
            ))),
            UCoord2D { x: _, y } if y >= self.dim.y => Err(Errors::DimError(format!(
                "y ({}) equal or above dimensional bound ({})",
                y, self.dim.y
            ))),
            UCoord2D { x, y } => Ok(x + self.dim.x * y),
        }
    }

    #[inline(always)]
    pub fn get(&self, coord: impl TryAsUCoord2D) -> Result<&T, Errors> {
        let idx = self.internal_index(coord)?;
        self.data
            .get(idx)
            .ok_or(Errors::DimError("Unexpected dim error".into()))
    }

    #[inline(always)]
    pub fn get_mut(&mut self, coord: impl TryAsUCoord2D) -> Result<&mut T, Errors> {
        let idx = self.internal_index(coord)?;
        self.data
            .get_mut(idx)
            .ok_or(Errors::DimError("Unexpected dim error".into()))
    }

    #[inline(always)]
    pub fn generate<F: FnMut(UCoord2D) -> Result<T, Errors>>(
        dim: UCoord2D,
        mut f: F,
    ) -> Result<Self, Errors> {
        let mut data: Vec<T> = Vec::with_capacity(dim.x * dim.y);
        for iy in 0..dim.y {
            for ix in 0..dim.x {
                let coord = UCoord2D { x: ix, y: iy };
                data.push(f(coord)?)
            }
        }
        Ok(Self { dim, data })
    }

    #[inline(always)]
    pub fn rotate_left(&self) -> Self {
        Self::generate(
            UCoord2D {
                x: self.dim.y,
                y: self.dim.x,
            },
            |UCoord2D { x: new_x, y: new_y }| {
                let old_coord = UCoord2D {
                    x: new_y,
                    y: self.dim.x - new_x - 1,
                };
                Ok(self.get(old_coord)?.to_owned())
            },
        )
        .unwrap()
    }

    #[inline(always)]
    pub fn sub_grid(&self, start: UCoord2D, dim: UCoord2D) -> Result<Self, Errors> {
        Self::generate(dim, |new_coord| {
            Ok(self.get(start + new_coord)?.clone())
        })
    }

    #[inline(always)]
    pub fn dim(&self) -> UCoord2D {
        self.dim
    }

    #[inline(always)]
    pub fn icoord_to_grid(&self, coord: ICoord2D) -> Option<UCoord2D> {
        let candidate: Option<UCoord2D> = coord.try_into().ok();
        match candidate {
            Some(UCoord2D { x, y: _ }) if x >= self.dim.x => None,
            Some(UCoord2D { x: _, y }) if y >= self.dim.y => None,
            val => val,
        }
    }
}
