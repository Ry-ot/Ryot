use glam::{UVec2, Vec2};
use std::sync::OnceLock;

#[cfg(feature = "bevy")]
pub mod camera;
pub mod grid;
#[cfg(feature = "lmdb")]
pub mod lmdb;
#[cfg(feature = "bevy")]
pub mod load;
pub mod map;

pub mod prelude {
    pub use crate::{
        grid::GRID_LAYER,
        map::directional::{CardinalDirection, Directional, OrdinalDirection},
        map::layer::{
            compute_z_transform, BottomLayer, Layer, LayerIter, Order, RelativeLayer,
            RelativeLayerIter,
        },
        map::map_tile::{MapTile, MapTileIter, MapTiles},
        map::position::{track_position_changes, PreviousPosition, TilePosition},
        map::sector::Sector,
        tile_offset, tile_size, TILE_SIZE,
    };

    #[cfg(feature = "bevy")]
    pub use crate::{
        camera::{
            cursor::{
                cursor_sliding_camera, draw_cursor_system, move_to_cursor, update_cursor_pos,
            },
            sector::update_camera_visible_sector,
        },
        grid::{spawn_grid, GridView},
        load::{prepare_sprite_layouts, prepare_sprite_meshes},
    };

    #[cfg(feature = "lmdb")]
    pub use crate::lmdb::*;
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
