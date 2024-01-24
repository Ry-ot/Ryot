use glam::{UVec2, Vec2, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
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

    /// Gets the projected position in the tile pos from a 2d display position.
    /// The display position is in pixels, the tile position is in tiles.
    pub fn get_tile_pos_from_display_pos_vec2(&self, cursor_pos: Vec2) -> Vec2 {
        Vec2::new(
            (cursor_pos.x / self.tile_size.x as f32) as i32 as f32,
            (-cursor_pos.y / self.tile_size.y as f32) as i32 as f32,
        )
    }

    /// Gets the projected position in the display pos from a 2d tile position.
    /// The display position is in pixels, the tile position is in tiles.
    /// The tile position is the position of the tile in the grid, not the position of the tile in the world.
    /// The tile position must always be positive, so if the display position is negative, it will return None.
    pub fn get_display_position_from_tile_pos_vec2(&self, tile_pos: Vec2) -> Option<Vec2> {
        if tile_pos.x < 0. || tile_pos.y < 0. {
            return None;
        }

        if tile_pos.x > self.columns as f32 || tile_pos.y > self.rows as f32 {
            return None;
        }

        // X grows the same for both tile and camera, so we just add the offset of half tile.
        // Y grows in opposite directions, so we need to invert Y and add the offset.
        let x = tile_pos.x * self.tile_size.x as f32 + (self.tile_size.x as f32 / 2.);
        let y = -tile_pos.y * self.tile_size.y as f32 - (self.tile_size.y as f32 / 2.);

        Some(Vec2::new(x, y))
    }

    /// Gets the projected position in the display pos from a 3d tile position.
    /// The display position is in pixels, the tile position is in tiles.
    /// The tile position is the position of the tile in the grid, not the position of the tile in the world.
    /// The tile position must always be positive, so if the display position is negative, it will return None.
    /// The z position is used to calculate the rendering order of the tile.
    pub fn get_display_position_from_tile_pos_vec3(&self, tile_pos: Vec3) -> Option<Vec3> {
        let pos = self.get_display_position_from_tile_pos_vec2(tile_pos.truncate())?;

        // z for 2d sprites define the rendering order, for 45 degrees top-down
        // perspective we always want right bottom items to be drawn on top.
        let z = tile_pos.z + (tile_pos.x + tile_pos.y) / u16::MAX as f32;

        Some(Vec3::from((pos, z)))
    }
}
