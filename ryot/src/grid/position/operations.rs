use crate::grid::TilePosition;
use glam::IVec2;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub, SubAssign};

impl Add<IVec2> for TilePosition {
    type Output = Self;
    #[inline]
    fn add(self, rhs: IVec2) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z)
    }
}

impl AddAssign<IVec2> for TilePosition {
    #[inline]
    fn add_assign(&mut self, rhs: IVec2) {
        *self = *self + rhs;
    }
}

impl Sub<IVec2> for TilePosition {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: IVec2) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z)
    }
}

impl SubAssign<IVec2> for TilePosition {
    #[inline]
    fn sub_assign(&mut self, rhs: IVec2) {
        *self = *self - rhs;
    }
}

impl PartialOrd<Self> for TilePosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TilePosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x
            .cmp(&other.x)
            .then(self.y.cmp(&other.y))
            .then(self.z.cmp(&other.z))
    }
}
