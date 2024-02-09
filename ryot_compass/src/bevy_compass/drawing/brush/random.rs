use crate::{Brush, BrushItem};
use ryot::position::TilePosition;

pub struct Random;

impl<B: BrushItem> From<Random> for Brush<B> {
    fn from(_: Random) -> Self {
        Brush::new(random::<B>)
    }
}

pub fn random<B: BrushItem>(size: i32, center: B) -> Vec<B> {
    let mut elements = vec![center];
    let center_pos = center.get_position();

    for _ in 0..size {
        let x = center_pos.x + rand::random::<i32>() % size;
        let y = center_pos.y + rand::random::<i32>() % size;
        elements.push(B::from_position(
            center,
            TilePosition::new(x, y, center_pos.z),
        ));
    }

    elements
}
