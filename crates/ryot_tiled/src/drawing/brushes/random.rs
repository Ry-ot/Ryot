use crate::prelude::*;

#[cfg(feature = "egui")]
use crate::include_svg;

pub struct Random;

const NAME: &str = "Random";
#[cfg(feature = "egui")]
const ICON: egui::ImageSource = include_svg!(
    r##"
    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="#fff6c2" viewBox="0 0 256 256"><path d="M237.66,178.34a8,8,0,0,1,0,11.32l-24,24A8,8,0,0,1,200,208V192a72.15,72.15,0,0,1-57.65-30.14l-41.72-58.4A56.1,56.1,0,0,0,55.06,80H32a8,8,0,0,1,0-16H55.06a72.12,72.12,0,0,1,58.59,30.15l41.72,58.4A56.08,56.08,0,0,0,200,176V160a8,8,0,0,1,13.66-5.66ZM143,107a8,8,0,0,0,11.16-1.86l1.2-1.67A56.08,56.08,0,0,1,200,80V96a8,8,0,0,0,13.66,5.66l24-24a8,8,0,0,0,0-11.32l-24-24A8,8,0,0,0,200,48V64a72.15,72.15,0,0,0-57.65,30.14l-1.2,1.67A8,8,0,0,0,143,107Zm-30,42a8,8,0,0,0-11.16,1.86l-1.2,1.67A56.1,56.1,0,0,1,55.06,176H32a8,8,0,0,0,0,16H55.06a72.12,72.12,0,0,0,58.59-30.15l1.2-1.67A8,8,0,0,0,113,149Z"></path></svg>
    "##
);

impl<B: BrushItem> From<Random> for Brush<B> {
    fn from(_: Random) -> Self {
        Brush::new(
            random::<B>,
            NAME,
            #[cfg(feature = "egui")]
            ICON,
        )
    }
}

pub fn random<B: BrushItem>(params: BrushParams<B>, center: B) -> Vec<B> {
    let mut elements = vec![center];
    let center_pos = center.get_position();

    let size = params.get_size(center);

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
