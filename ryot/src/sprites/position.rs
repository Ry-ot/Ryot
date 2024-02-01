use std::ops::Deref;

use glam::{IVec3, UVec2, Vec2, Vec3};
use serde::{Deserialize, Serialize};

/// A 2d position in the tile grid. This is is not the position of the tile on
/// the screen, because it doesn't take into account the tile size. Z is used to
/// calculate the rendering order of the tile.
#[derive(Eq, PartialEq, Deserialize, Serialize, Default, Clone, Copy, Debug)]
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
        self.clamp(Self::MIN.0, Self::MAX.0).truncate() == self.truncate()
    }
}

impl Deref for TilePosition {
    type Target = IVec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec2> for TilePosition {
    fn from(screen_pos: Vec2) -> Self {
        Self(
            (screen_pos / tile_size().as_vec2())
                .ceil()
                .as_ivec2()
                .extend(0),
        )
    }
}

impl From<TilePosition> for Vec2 {
    fn from(tile_pos: TilePosition) -> Self {
        (tile_pos.as_vec3().truncate() + TilePosition::BOTTOM_RIGHT_OFFSET) * tile_size().as_vec2()
    }
}

impl From<TilePosition> for Vec3 {
    fn from(tile_pos: TilePosition) -> Self {
        // z for 2d sprites define the rendering order, for 45 degrees top-down
        // perspective we always want right bottom items to be drawn on top.
        Vec2::from(tile_pos)
            .extend((tile_pos.z + (tile_pos.x + tile_pos.y) / u16::MAX as i32) as f32)
    }
}

#[cfg(not(test))]
use crate::CONTENT_CONFIG;

#[cfg(not(test))]
pub fn tile_size() -> UVec2 {
    CONTENT_CONFIG.sprite_sheet.tile_size
}

#[cfg(test)]
pub fn tile_size() -> UVec2 {
    UVec2::new(32, 32)
}
