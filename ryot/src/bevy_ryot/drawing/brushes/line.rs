use crate::bevy_ryot::drawing::brushes::*;
#[cfg(feature = "egui")]
use crate::include_svg;
use glam::IVec2;

pub struct Line;

const NAME: &str = "Line";
#[cfg(feature = "egui")]
const ICON: egui::ImageSource = include_svg!(
    r##"
    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="#fff6c2" viewBox="0 0 256 256"><path d="M214.64,86.62a32.07,32.07,0,0,1-38.89,4.94L91.56,175.75a32,32,0,1,1-50.2-6.37h0a32.06,32.06,0,0,1,38.89-4.94l84.19-84.19a32,32,0,1,1,50.2,6.37Z"></path></svg>
    "##
);

impl<B: BrushItem> From<Line> for Brush<B> {
    fn from(_: Line) -> Self {
        Brush::new(
            line::<B>,
            NAME,
            #[cfg(feature = "egui")]
            ICON,
        )
    }
}

pub fn line<B: BrushItem>(params: BrushParams<B>, center: B) -> Vec<B> {
    let mut center_pos = center.get_position();
    let mut elements = Vec::new();

    let start_pos = match params {
        BrushParams::Size(size) => {
            center_pos += IVec2::new(size, 0);
            center_pos - IVec2::new(size, 0)
        }
        BrushParams::Position(pos) => pos,
        BrushParams::Element(e) => e.get_position(),
    };

    for pos in bresenhams_line(start_pos, center_pos) {
        elements.push(B::from_position(center, pos));
    }

    elements
}

fn bresenhams_line(a: TilePosition, b: TilePosition) -> Vec<TilePosition> {
    let mut points = Vec::new();

    let mut current = a;
    let dx = (b.x - a.x).abs();
    let dy = -(b.y - a.y).abs();
    let sx = if a.x < b.x { 1 } else { -1 };
    let sy = if a.y < b.y { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        points.push(current);
        if current == b {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            current.x += sx;
        }
        if e2 <= dx {
            err += dx;
            current.y += sy;
        }
    }

    points
}
