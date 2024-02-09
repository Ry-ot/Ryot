use crate::bevy_ryot::drawing::brushes::*;

pub struct Square;

impl<B: BrushItem> From<Square> for Brush<B> {
    fn from(_: Square) -> Self {
        Brush::new(square::<B>)
    }
}

pub fn square<B: BrushItem>(size: i32, center: B) -> Vec<B> {
    let mut elements = Vec::new();
    let center_pos = center.get_position();

    for x in center_pos.x.saturating_sub(size)..=center_pos.x.saturating_add(size) {
        for y in center_pos.y.saturating_sub(size)..=center_pos.y.saturating_add(size) {
            elements.push(B::from_position(
                center,
                TilePosition::new(x, y, center_pos.z),
            ));
        }
    }

    elements
}
