use crate::bevy_ryot::drawing::brushes::*;
#[cfg(feature = "egui")]
use crate::include_svg;

pub struct Round;

const NAME: &str = "Round";
#[cfg(feature = "egui")]
const ICON: egui::ImageSource = include_svg!(
    r##"
    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="#fff6c2" viewBox="0 0 256 256"><path d="M232,128A104,104,0,1,1,128,24,104.13,104.13,0,0,1,232,128Z"></path></svg>
    "##
);

impl<B: BrushItem> From<Round> for Brush<B> {
    fn from(_: Round) -> Self {
        Brush::new(
            round::<B>,
            NAME,
            #[cfg(feature = "egui")]
            ICON,
        )
    }
}

pub fn round<B: BrushItem>(params: BrushParams<B>, center: B) -> Vec<B> {
    let mut elements = Vec::new();
    let center_pos = center.get_position();

    let size = params.get_size(center);

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
