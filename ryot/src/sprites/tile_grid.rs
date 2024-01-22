/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */
use glam::{Vec2, Vec3};

/// A grid of tiles, columns and rows are capped at u16::MAX because of performance reasons.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TileGrid {
    pub columns: u16,
    pub rows: u16,
    // pub tile_size: UVec2,
}

impl TileGrid {
    pub fn new(columns: u16, rows: u16) -> Self {
        TileGrid { columns, rows }
    }

    pub fn get_tile_count(&self) -> u32 {
        self.columns as u32 * self.rows as u32
    }
}

pub fn normalize_tile_pos_to_sprite_pos(tile_pos: Vec2) -> Option<Vec2> {
    if tile_pos.x < 0. || tile_pos.y < 0. {
        return None;
    }

    if tile_pos.x > u16::MAX as f32 || tile_pos.y > u16::MAX as f32 {
        return None;
    }

    // X grows the same for both tile and camera, so we just add the offset of half tile.
    // Y grows in opposite directions, so we need to invert Y and add the offset.
    let x = tile_pos.x * 32. + (32. / 2.);
    let y = -tile_pos.y * 32. - (32. / 2.);

    Some(Vec2::new(x, y))
}

pub fn normalize_tile_pos_to_sprite_pos_with_z(tile_pos: Vec3) -> Option<Vec3> {
    let Some(pos) = normalize_tile_pos_to_sprite_pos(tile_pos.truncate()) else {
        return None;
    };

    // z for 2d sprites define the rendering order, for 45 degrees top-down
    // perspective we always want right bottom items to be drawn on top.
    let z = tile_pos.z + (pos.x - pos.y) / u16::MAX as f32;

    Some(Vec3::from((pos, z)))
}
