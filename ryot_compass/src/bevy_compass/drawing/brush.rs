use bevy::prelude::*;
use ryot::prelude::{drawing::*, position::*};

pub trait Brush {
    fn to_paint(&self, center: DrawingBundle) -> Vec<DrawingBundle>;
}

pub struct SingleTileBrush;
impl Brush for SingleTileBrush {
    fn to_paint(&self, center: DrawingBundle) -> Vec<DrawingBundle> {
        vec![center]
    }
}

#[derive(Debug, Default, Resource, Deref, Reflect, DerefMut)]
pub struct SquareBrush(u8);

impl SquareBrush {
    pub fn new(size: u8) -> Self {
        Self(size.clamp(1, 50))
    }
}

impl Brush for SquareBrush {
    fn to_paint(&self, center: DrawingBundle) -> Vec<DrawingBundle> {
        let mut positions = Vec::new();
        let DrawingBundle {
            layer,
            tile_pos,
            appearance,
            visibility,
            ..
        } = center;

        let side = self.0 as i32;

        for x in tile_pos.x.saturating_sub(side)..=tile_pos.x.saturating_add(side) {
            for y in tile_pos.y.saturating_sub(side)..=tile_pos.y.saturating_add(side) {
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

#[derive(Debug, Default, Resource, Deref, Reflect, DerefMut)]
pub struct RoundBrush(u8);

impl RoundBrush {
    pub fn new(size: u8) -> Self {
        Self(size.clamp(1, 50))
    }
}

impl Brush for RoundBrush {
    fn to_paint(&self, center: DrawingBundle) -> Vec<DrawingBundle> {
        let mut positions = Vec::new();
        let DrawingBundle {
            layer,
            tile_pos,
            appearance,
            visibility,
            ..
        } = center;

        let radius = self.0 as i32;

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
