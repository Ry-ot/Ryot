use crate::prelude::*;
use glam::{Vec2, Vec3};

#[cfg(feature = "bevy")]
use bevy_math::bounding::{Aabb3d, BoundingSphere};
#[cfg(feature = "bevy")]
use bevy_transform::prelude::*;

impl TilePosition {
    pub fn to_vec3(self, layer: &Layer) -> Vec3 {
        Vec2::from(self).extend(compute_z_transform(&self, layer))
    }
}

impl From<Vec2> for TilePosition {
    fn from(screen_pos: Vec2) -> Self {
        Self(
            ((screen_pos - TilePosition::BOTTOM_RIGHT_OFFSET) / tile_size().as_vec2())
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

impl From<&TilePosition> for Vec2 {
    fn from(tile_pos: &TilePosition) -> Self {
        Vec2::from(*tile_pos)
    }
}

#[cfg(feature = "bevy")]
impl From<Transform> for TilePosition {
    fn from(transform: Transform) -> Self {
        transform.translation.truncate().into()
    }
}

#[cfg(feature = "bevy")]
impl From<&Transform> for TilePosition {
    fn from(transform: &Transform) -> Self {
        TilePosition::from(*transform)
    }
}

/// Converts `TilePosition` to `Aabb3d` (Axis Aligned Bounding Box 3D).
/// This conversion allows `TilePosition` to be used in spatial queries and collision detection.
#[cfg(feature = "bevy")]
impl From<TilePosition> for Aabb3d {
    fn from(tile_pos: TilePosition) -> Self {
        let vec3 = Vec3::new(tile_pos.x as f32, tile_pos.y as f32, tile_pos.z as f32);
        Aabb3d::new(vec3, Vec3::new(0.35, 0.35, 0.))
    }
}

/// Converts `TilePosition` to `BoundingSphere`.
/// This conversion allows `TilePosition` to be used in spatial queries and collision detection
/// with spherical bounds.
#[cfg(feature = "bevy")]
impl From<TilePosition> for BoundingSphere {
    fn from(tile_pos: TilePosition) -> Self {
        let vec3 = Vec3::new(tile_pos.x as f32, tile_pos.y as f32, tile_pos.z as f32);
        BoundingSphere::new(vec3, 0.55)
    }
}
