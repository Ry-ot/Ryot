use crate::{bevy_ryot::drawing::brushes::*, include_svg};

#[derive(Clone, Copy)]
pub struct Square;

impl<B: BrushItem> From<Square> for Brush<B> {
    fn from(_: Square) -> Self {
        Brush::new(square::<B>)
    }
}

impl SelectableTool for Square {
    fn name(&self) -> &str {
        "Square"
    }

    fn icon(&self) -> ImageSource {
        include_svg!(
            r##"
            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="#fff6c2" viewBox="0 0 256 256"><rect x="32" y="32" width="192" height="192" rx="16"></rect></svg>
            "##
        )
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
