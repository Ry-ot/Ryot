use crate::layer::compute_z_transform;
use crate::{Layer, SpriteLayout};
use derive_more::{Add, Sub};
use glam::{IVec3, Vec2, Vec3};
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Deref, DerefMut};

use crate::bevy_ryot::elevation::Elevation;
use crate::grid::tile_size;
#[cfg(feature = "bevy")]
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

mod conversion;

mod interactions;

mod operations;

#[cfg(feature = "pathfinding")]
mod pathfinding;

mod previous;

pub use previous::*;

/// A 2d position in the tile grid. This is not the position of the tile on
/// the screen, because it doesn't take into account the tile size. Z is used to
/// calculate the rendering order of the tile.
#[derive(Eq, PartialEq, Deserialize, Serialize, Default, Clone, Copy, Debug, Hash, Add, Sub)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
pub struct TilePosition(pub IVec3);

impl TilePosition {
    /// The minimum possible tile position. This has to be something that when multiplied by the tile size does not overflow f32.
    pub const MIN: TilePosition = TilePosition(IVec3::new(i16::MIN as i32, i16::MIN as i32, 0));

    /// The maximum possible tile position. This has to be something that when multiplied by the tile size does not overflow f32.
    pub const MAX: TilePosition = TilePosition(IVec3::new(i16::MAX as i32, i16::MAX as i32, 0));

    pub const ZERO: TilePosition = TilePosition(IVec3::ZERO);

    pub const BOTTOM_RIGHT_OFFSET: Vec2 = Vec2::new(0., -1.);

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self(IVec3::new(x, y, z))
    }

    pub fn with_z(self, z: i32) -> Self {
        Self(self.0.truncate().extend(z))
    }

    pub fn is_valid(self) -> bool {
        self.deref().clamp(Self::MIN.0, Self::MAX.0).truncate() == self.truncate()
    }

    // TODO: Probably not living here.
    pub fn to_elevated_translation(
        self,
        layout: SpriteLayout,
        layer: Layer,
        elevation: Elevation,
    ) -> Vec3 {
        let anchor = Vec2::new(
            (elevation.elevation).clamp(0.0, 1.0),
            (-elevation.elevation).clamp(-1.0, 0.0),
        );
        self.to_vec3(&layer)
            - (SpriteLayout::OneByOne.get_size(&tile_size()).as_vec2() * anchor).extend(0.)
            - (layout.get_size(&tile_size()).as_vec2() * Vec2::new(0.5, -0.5)).extend(0.)
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

#[cfg(test)]
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
