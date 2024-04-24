use crate::prelude::*;

#[cfg(feature = "egui")]
use crate::include_svg;

pub struct Rectangle;

const NAME: &str = "Rectangle";
#[cfg(feature = "egui")]
const ICON: egui::ImageSource = include_svg!(
    r##"
    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="#fff6c2" viewBox="0 0 256 256"><rect x="24" y="40" width="208" height="176" rx="16"></rect></svg>
    "##
);

impl<B: BrushItem> From<Rectangle> for Brush<B> {
    fn from(_: Rectangle) -> Self {
        Brush::new(
            rectangle::<B>,
            NAME,
            #[cfg(feature = "egui")]
            ICON,
        )
    }
}

pub fn rectangle<B: BrushItem>(params: BrushParams<B>, center: B) -> Vec<B> {
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
