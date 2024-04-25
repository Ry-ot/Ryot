#![feature(fn_traits)]
#![feature(unboxed_closures)]
use glam::{UVec2, Vec2};
use std::sync::OnceLock;

#[cfg(feature = "bevy")]
pub mod camera;
#[cfg(feature = "bevy")]
pub mod drawing;
pub mod map;
pub mod movement;
#[cfg(feature = "bevy")]
pub mod object;

pub mod prelude {
    pub use crate::{
        map::directional::{CardinalDirection, Directional, OrdinalDirection},
        map::grid::GRID_LAYER,
        map::layer::{
            compute_z_transform, BottomLayer, Layer, LayerIter, Order, RelativeLayer,
            RelativeLayerIter,
        },
        map::map_tile::{MapTile, MapTileIter, MapTiles},
        map::position::{PreviousPosition, TilePosition},
        map::sector::Sector,
        movement::SpriteMovement,
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
        drawing::*,
        map::elevation::{apply_elevation, elevate_position, initialize_elevation},
        map::grid::{spawn_grid, GridView},
        map::position::{
            systems::{
                finish_position_animation, move_sprites_with_animation, update_sprite_position,
            },
            track_position_changes,
        },
        object::{GameObjectBundle, LoadObjects},
    };

    #[cfg(feature = "lmdb")]
    pub use crate::map::lmdb::{
        systems::{
            compact_map, init_tiles_db, load_area, read_area, reload_visible_area, LmdbCompactor,
            LmdbEnv,
        },
        *,
    };

    #[cfg(feature = "egui")]
    pub use crate::include_svg;

    #[cfg(feature = "debug")]
    pub use crate::map::position::systems::{
        debug_sprite_position, debug_y_offset, PositionDebugText,
    };
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
