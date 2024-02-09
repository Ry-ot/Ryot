use crate::{Brush, BrushItem};
use ryot::position::TilePosition;

pub struct Round;

impl<B: BrushItem> From<Round> for Brush<B> {
    fn from(_: Round) -> Self {
        Brush::new(round::<B>)
    }
}

pub fn round<B: BrushItem>(size: i32, center: B) -> Vec<B> {
    let mut elements = Vec::new();
    let center_pos = center.get_position();

    for x in center_pos.x.saturating_sub(size)..=center_pos.x.saturating_add(size) {
        for y in center_pos.y.saturating_sub(size)..=center_pos.y.saturating_add(size) {
            let distance = center_pos.distance(TilePosition::new(x, y, center_pos.z));
            if distance <= size as f32 {
                elements.push(B::from_position(
                    center,
                    TilePosition::new(x, y, center_pos.z),
                ));
            }
        }
    }

    elements
}
