use crate::{Brush, BrushAction};
use bevy::prelude::*;
use ryot::bevy_ryot::drawing::{DrawingBundle, Tile};
use ryot::position::TilePosition;

#[derive(Debug, Deref, Eq, PartialEq, Reflect, DerefMut, Copy, Clone, Hash)]
pub struct DiamondBrush(i32);

impl DiamondBrush {
    pub fn new(size: i32) -> Self {
        Self(size.abs().clamp(1, 50))
    }
}

impl From<DiamondBrush> for Brush {
    fn from(brush: DiamondBrush) -> Self {
        Brush::Diamond(brush)
    }
}

impl Default for DiamondBrush {
    fn default() -> Self {
        Self::new(3)
    }
}

impl BrushAction for DiamondBrush {
    fn apply(&self, center: DrawingBundle) -> Vec<DrawingBundle> {
        let mut positions = Vec::new();
        let DrawingBundle {
            layer,
            tile_pos,
            appearance,
            visibility,
            ..
        } = center;

        let Self(size) = *self;

        for x_offset in -size..=size {
            for y_offset in -size..=size {
                if x_offset.abs() + y_offset.abs() <= size {
                    let new_pos =
                        TilePosition::new(tile_pos.x + x_offset, tile_pos.y + y_offset, tile_pos.z);
                    positions.push(DrawingBundle {
                        layer,
                        tile_pos: new_pos,
                        appearance,
                        visibility,
                        tile: Tile,
                    });
                }
            }
        }

        positions
    }
}
