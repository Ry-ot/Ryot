use glam::{IVec2, UVec2, Vec2, Vec3};
use serde::Deserialize;

#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub struct TileGrid {
    pub columns: u32,
    pub rows: u32,
    pub tile_size: UVec2,
}

/// A grid of tiles, columns and rows are capped at u16::MAX because of performance reasons.
impl Default for TileGrid {
    fn default() -> Self {
        TileGrid {
            columns: u16::MAX as u32,
            rows: u16::MAX as u32,
            tile_size: UVec2::new(32, 32),
        }
    }
}

impl TileGrid {
    pub fn from_grid_size(columns: u32, rows: u32) -> Self {
        TileGrid {
            columns,
            rows,
            ..Default::default()
        }
    }

    pub fn from_tile_size(tile_size: UVec2) -> Self {
        TileGrid {
            tile_size,
            ..Default::default()
        }
    }

    pub fn with_grid_size(mut self, columns: u32, rows: u32) -> Self {
        self.columns = columns;
        self.rows = rows;
        self
    }

    pub fn with_tile_size(mut self, tile_size: UVec2) -> Self {
        self.tile_size = tile_size;
        self
    }

    pub fn get_tile_count(&self) -> u32 {
        self.columns * self.rows
    }

    pub fn get_width(&self) -> u32 {
        self.columns * self.tile_size.x
    }

    pub fn get_height(&self) -> u32 {
        self.rows * self.tile_size.y
    }

    pub fn get_size(&self) -> UVec2 {
        UVec2::new(self.get_width(), self.get_height())
    }

    /// Gets the min/max screen positions in pixels.
    /// The min is the bottom left corner, the max is the top right corner.
    pub fn get_bounds_screen(&self) -> (Vec2, Vec2) {
        let (min, max) = self.get_bounds_tiles();
        (
            Vec2::new(min.x as f32 - 1., min.y as f32) * self.tile_size.as_vec2(),
            Vec2::new(max.x as f32, max.y as f32 + 1.) * self.tile_size.as_vec2(),
        )
    }

    /// Gets the min/max tile positions in the grid.
    /// The min is the bottom left tile, the max is the top right tile.
    pub fn get_bounds_tiles(&self) -> (IVec2, IVec2) {
        (
            IVec2::new(-(self.columns as i32 / 2) + 1, -(self.rows as i32 / 2)),
            IVec2::new(self.columns as i32 / 2, self.rows as i32 / 2 - 1),
        )
    }

    /// Gets the projected position in the tile pos from a 2d display position.
    /// The display position is in pixels, the tile position is in tiles.
    pub fn get_tile_pos_from_display_pos(&self, screen_pos: Vec2) -> Vec2 {
        let vec = screen_pos / self.tile_size.as_vec2();
        let (min, max) = self.get_bounds_tiles();
        Vec2::new(
            if vec.x < 0. {
                vec.x.trunc()
            } else {
                vec.x.ceil()
            },
            if vec.y >= 0. {
                vec.y.trunc()
            } else {
                vec.y.trunc() - 1.
            },
        )
        .clamp(min.as_vec2(), max.as_vec2())
    }

    /// Gets the projected position in the display pos from a 2d tile position.
    /// The display position is in pixels, the tile position is in tiles.
    /// The tile position is the position of the tile in the grid, not the position of the tile in the world.
    /// The tile position must always be positive, so if the display position is negative, it will return None.
    /// The z position is used to calculate the rendering order of the tile.
    pub fn get_display_position_from_tile_pos(&self, tile_pos: Vec3) -> Option<Vec3> {
        let (min, max) = self.get_bounds_tiles();
        if tile_pos.truncate().clamp(min.as_vec2(), max.as_vec2()) != tile_pos.truncate() {
            return None;
        }

        Some(Vec3::new(
            tile_pos.x * self.tile_size.x as f32,
            tile_pos.y * self.tile_size.y as f32,
            // z for 2d sprites define the rendering order, for 45 degrees top-down
            // perspective we always want right bottom items to be drawn on top.
            tile_pos.z + (tile_pos.x + tile_pos.y) / u16::MAX as f32,
        ))
    }
}
