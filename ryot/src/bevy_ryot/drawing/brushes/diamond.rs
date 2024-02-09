use crate::bevy_ryot::drawing::brushes::*;

pub struct Diamond;

impl<B: BrushItem> From<Diamond> for Brush<B> {
    fn from(_: Diamond) -> Self {
        Brush::new(diamond::<B>)
    }
}

pub fn diamond<B: BrushItem>(size: i32, center: B) -> Vec<B> {
    let mut elements = Vec::new();
    let center_pos = center.get_position();

    for x_offset in -size..=size {
        for y_offset in -size..=size {
            if x_offset.abs() + y_offset.abs() <= size {
                elements.push(B::from_position(
                    center,
                    TilePosition::new(
                        center_pos.x + x_offset,
                        center_pos.y + y_offset,
                        center_pos.z,
                    ),
                ));
            }
        }
    }

    elements
}
