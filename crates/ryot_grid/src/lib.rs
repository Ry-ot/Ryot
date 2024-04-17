use glam::{UVec2, Vec2};
use std::sync::OnceLock;

pub mod directional;
pub mod layer;
pub mod map;
pub mod position;
pub mod sector;

pub mod prelude {
    pub use crate::{
        directional::{CardinalDirection, Directional, OrdinalDirection},
        layer::{
            compute_z_transform, BottomLayer, Layer, LayerIter, Order, RelativeLayer,
            RelativeLayerIter,
        },
        map::{MapTile, MapTileIter, MapTiles},
        position::{PreviousPosition, TilePosition},
        sector::Sector,
        tile_offset, tile_size, TILE_SIZE,
    };

    #[cfg(feature = "bevy")]
    pub use crate::position::track_position_changes;
}

pub static TILE_SIZE: OnceLock<UVec2> = OnceLock::new();

#[cfg(test)]
pub fn tile_size() -> UVec2 {
    UVec2::new(32, 32)
}

#[cfg(not(test))]
pub fn tile_size() -> UVec2 {
    *TILE_SIZE.get().expect("TILE_SIZE not initialized")
}

pub fn tile_offset() -> Vec2 {
    Vec2::new(-1., 1.) * tile_size().as_vec2()
}
