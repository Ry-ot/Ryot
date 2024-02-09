use crate::Brush;
use ryot::bevy_ryot::drawing::{DrawingBundle, Tile};
use ryot::position::TilePosition;

pub struct Square;

impl From<Square> for Brush {
    fn from(_: Square) -> Self {
        Brush::new(square)
    }
}

pub fn square(size: i32, center: DrawingBundle) -> Vec<DrawingBundle> {
    let mut positions = Vec::new();
    let DrawingBundle {
        layer,
        tile_pos,
        appearance,
        visibility,
        ..
    } = center;

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
