use derive_more::{Add, Sub};
use glam::{IVec3, Vec2};
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

use serde::{Deserialize, Serialize};

mod conversion;

mod interactions;

mod operations;

mod previous;
pub use previous::*;
use ryot_core::game::Point;

#[cfg(feature = "bevy")]
pub mod systems;

/// A 2d position in the tile grid. This is not the position of the tile on
/// the screen, because it doesn't take into account the tile size. Z is used to
/// calculate the rendering order of the tile.
#[derive(Eq, PartialEq, Deserialize, Serialize, Default, Clone, Copy, Debug, Hash, Add, Sub)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "pathfinding", derive(ryot_derive::Pathable))]
pub struct TilePosition(pub IVec3);

impl TilePosition {
    /// The minimum possible tile position. This has to be something that when multiplied by the tile size does not overflow f32.
    pub const MIN: TilePosition = TilePosition(IVec3::new(i16::MIN as i32, i16::MIN as i32, 0));

    /// The maximum possible tile position. This has to be something that when multiplied by the tile size does not overflow f32.
    pub const MAX: TilePosition = TilePosition(IVec3::new(i16::MAX as i32, i16::MAX as i32, 0));

    pub const ZERO: TilePosition = TilePosition(IVec3::ZERO);

    pub const BOTTOM_RIGHT_OFFSET: Vec2 = Vec2::new(0., -1.);

    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self(IVec3::new(x, y, z))
    }

    pub fn with_z(self, z: i32) -> Self {
        Self(self.0.truncate().extend(z))
    }

    pub fn is_valid(self) -> bool {
        self.deref().clamp(Self::MIN.0, Self::MAX.0).truncate() == self.truncate()
    }
}

impl fmt::Display for TilePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Deref for TilePosition {
    type Target = IVec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TilePosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// TilePosition is a spatial Point, this trait guarantees good integration with the rest of the
/// RyOT ecosystem, such as pathfinding, ray casting, and other spatial algorithms.
impl Point for TilePosition {
    fn generate(x: i32, y: i32, z: i32) -> Self {
        TilePosition::new(x, y, z)
    }

    fn coordinates(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }
}

#[cfg(feature = "test-utils")]
impl quickcheck::Arbitrary for TilePosition {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self::new(
            i16::arbitrary(g) as i32,
            i16::arbitrary(g) as i32,
            i8::arbitrary(g) as i32,
        )
    }
}

#[cfg(test)]
mod tests;
