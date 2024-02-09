use crate::Brush;
use ryot::bevy_ryot::drawing::{DrawingBundle, Tile};
use ryot::position::TilePosition;

pub struct Diamond;

impl From<Diamond> for Brush {
    fn from(_: Diamond) -> Self {
        Brush::new(diamond)
    }
}

pub fn diamond(size: i32, center: DrawingBundle) -> Vec<DrawingBundle> {
    let mut positions = Vec::new();
    let DrawingBundle {
        layer,
        tile_pos,
        appearance,
        visibility,
        ..
    } = center;

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
