#[cfg(not(test))]
use crate::TILE_SIZE;

use glam::{UVec2, Vec2};

pub mod position;
pub use position::*;

pub mod sector;
pub use sector::*;

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
