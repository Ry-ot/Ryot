use crate::Brush;
use bevy::prelude::{Deref, DerefMut, Reflect, Resource};
use ryot::bevy_ryot::drawing::{DrawingBundle, Tile};
use ryot::position::TilePosition;

#[derive(Debug, Default, Resource, Deref, Reflect, DerefMut)]
pub struct RoundBrush(i32);

impl RoundBrush {
    pub fn new(size: i32) -> Self {
        Self(size.abs().clamp(1, 50))
    }
}

impl Brush for RoundBrush {
    fn apply(&self, center: DrawingBundle) -> Vec<DrawingBundle> {
        let mut positions = Vec::new();
        let DrawingBundle {
            layer,
            tile_pos,
            appearance,
            visibility,
            ..
        } = center;

        let Self(radius) = *self;

        for x in tile_pos.x.saturating_sub(radius)..=tile_pos.x.saturating_add(radius) {
            for y in tile_pos.y.saturating_sub(radius)..=tile_pos.y.saturating_add(radius) {
                let distance = tile_pos.distance(TilePosition::new(x, y, tile_pos.z));
                if distance <= self.0 as f32 {
                    positions.push(DrawingBundle {
                        layer,
                        tile_pos: TilePosition::new(x, y, tile_pos.z),
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
