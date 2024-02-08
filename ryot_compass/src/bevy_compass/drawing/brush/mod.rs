use ryot::prelude::drawing::*;

mod diamond;
pub use diamond::DiamondBrush;

mod round;
pub use round::RoundBrush;

mod square;
pub use square::SquareBrush;

pub trait Brush {
    fn apply(&self, center: DrawingBundle) -> Vec<DrawingBundle>;
}

pub enum BrushType {
    Round(RoundBrush),
    Square(SquareBrush),
    Diamond(DiamondBrush),
    // Custom(dyn Fn(DrawingBundle) -> Vec<DrawingBundle>),
}

pub struct SingleTileBrush;
impl Brush for SingleTileBrush {
    fn apply(&self, center: DrawingBundle) -> Vec<DrawingBundle> {
        vec![center]
    }
}

impl Brush for dyn Fn(DrawingBundle) -> Vec<DrawingBundle> {
    fn apply(&self, center: DrawingBundle) -> Vec<DrawingBundle> {
        self(center)
    }
}
