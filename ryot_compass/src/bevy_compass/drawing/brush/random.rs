use crate::Brush;
use ryot::bevy_ryot::drawing::{DrawingBundle, Tile};
use ryot::position::TilePosition;

pub struct Random;

impl From<Random> for Brush {
    fn from(_: Random) -> Self {
        Brush::new(random)
    }
}

pub fn random(size: i32, center: DrawingBundle) -> Vec<DrawingBundle> {
    let mut positions = vec![center];
    let DrawingBundle {
        layer,
        tile_pos,
        appearance,
        visibility,
        ..
    } = center;

    for _ in 0..size {
        let x = tile_pos.x + rand::random::<i32>() % size;
        let y = tile_pos.y + rand::random::<i32>() % size;
        positions.push(DrawingBundle {
            layer,
            tile_pos: TilePosition::new(x, y, tile_pos.z),
            appearance,
            visibility,
            tile: Tile,
        });
    }

    positions
}
