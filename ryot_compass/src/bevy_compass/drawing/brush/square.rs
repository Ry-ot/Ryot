use crate::BrushAction;
use bevy::prelude::*;
use ryot::bevy_ryot::drawing::{DrawingBundle, Tile};
use ryot::position::TilePosition;

#[derive(Debug, Deref, Eq, PartialEq, Reflect, DerefMut, Copy, Clone, Hash)]
pub struct SquareBrush(pub i32);

impl BrushAction for SquareBrush {
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

        for x in tile_pos.x.saturating_sub(size)..=tile_pos.x.saturating_add(size) {
            for y in tile_pos.y.saturating_sub(size)..=tile_pos.y.saturating_add(size) {
                positions.push(DrawingBundle {
                    layer,
                    tile_pos: TilePosition::new(x, y, tile_pos.z),
                    appearance,
                    visibility,
                    tile: Tile,
                });
            }
        }

        positions
    }
}
