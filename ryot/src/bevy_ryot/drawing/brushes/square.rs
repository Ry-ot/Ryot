use crate::{bevy_ryot::drawing::brushes::*, include_svg};

pub struct Square;

const NAME: &str = "Square";
const ICON: ImageSource = include_svg!(
    r##"
    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="#fff6c2" viewBox="0 0 256 256"><rect x="32" y="32" width="192" height="192" rx="16"></rect></svg>
    "##
);

impl<B: BrushItem> From<Square> for Brush<B> {
    fn from(_: Square) -> Self {
        Brush::new(square::<B>, NAME, ICON)
    }
}

pub fn square<B: BrushItem>(params: BrushParams<B>, center: B) -> Vec<B> {
    let mut elements = Vec::new();
    let center_pos = center.get_position();

    let (x_range, y_range) = params.get_ranges(center);

    for x in x_range.clone() {
        for y in y_range.clone() {
            elements.push(B::from_position(
                center,
                TilePosition::new(x, y, center_pos.z),
            ));
        }
    }

    elements
}
