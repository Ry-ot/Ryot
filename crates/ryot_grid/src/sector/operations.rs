use crate::prelude::TilePosition;
use crate::sector::Sector;
use std::ops::{Div, DivAssign, Mul, MulAssign, Sub};

impl Sub<Sector> for Sector {
    type Output = Vec<Sector>;

    fn sub(self, rhs: Sector) -> Self::Output {
        if self == rhs {
            return Vec::new();
        }

        if rhs == Sector::ZERO {
            return vec![self];
        }

        if self == Sector::ZERO {
            return vec![rhs];
        }

        let mut result = Vec::new();

        // Left area (corrected to ensure no overlap and accurate representation)
        if rhs.min.x < self.min.x {
            result.push(Self {
                min: TilePosition::new(rhs.min.x, rhs.min.y, 0),
                max: TilePosition::new(self.min.x, rhs.max.y, 0),
            });
        }

        // Bottom area
        if rhs.min.y < self.min.y {
            result.push(Self {
                min: TilePosition::new(self.min.x, rhs.min.y, 0),
                max: TilePosition::new(self.max.x, self.min.y, 0),
            });
        }

        // Right area (corrected for the same reason as the left area)
        if rhs.max.x > self.max.x {
            result.push(Self {
                min: TilePosition::new(self.max.x, rhs.min.y, 0),
                max: TilePosition::new(rhs.max.x, rhs.max.y, 0),
            });
        }

        // Top area
        if rhs.max.y > self.max.y {
            result.push(Self {
                min: TilePosition::new(self.min.x, self.max.y, 0),
                max: TilePosition::new(self.max.x, rhs.max.y, 0),
            });
        }

        result
    }
}

impl Mul<f32> for Sector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let delta = (self.size().as_vec2() * (rhs - 1.0)) / 2.0;
        let delta = delta.as_ivec2();

        Sector::new(self.min - delta, self.max + delta)
    }
}

impl MulAssign<f32> for Sector {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Sector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f32> for Sector {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
