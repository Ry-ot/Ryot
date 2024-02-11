use crate::{bevy_ryot::drawing::brushes::*, include_svg};

#[derive(Clone, Copy)]
pub struct Diamond;

impl<B: BrushItem> From<Diamond> for Brush<B> {
    fn from(_: Diamond) -> Self {
        Brush::new(diamond::<B>)
    }
}

impl SelectableTool for Diamond {
    fn name(&self) -> &str {
        "Diamond"
    }

    fn icon(&self) -> ImageSource {
        include_svg!(
            r##"
            <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="#fff6c2" viewBox="0 0 256 256"><path d="M240,128a15.85,15.85,0,0,1-4.67,11.28l-96.05,96.06a16,16,0,0,1-22.56,0h0l-96-96.06a16,16,0,0,1,0-22.56l96.05-96.06a16,16,0,0,1,22.56,0l96.05,96.06A15.85,15.85,0,0,1,240,128Z"></path></svg>
            "##
        )
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
