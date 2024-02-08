use bevy::prelude::*;
use ryot::prelude::drawing::*;

mod diamond;
pub use diamond::DiamondBrush;

mod round;
pub use round::RoundBrush;

mod square;
pub use square::SquareBrush;

mod systems;
pub use systems::update_brush;

pub trait BrushAction: Eq + PartialEq + Clone + Reflect + Send + Sync + 'static {
    fn apply(&self, center: DrawingBundle) -> Vec<DrawingBundle>;
}

#[derive(Component, Eq, PartialEq, Reflect, Copy, Clone, Hash)]
pub enum Brush {
    SingleTile,
    Round(RoundBrush),
    Square(SquareBrush),
    Diamond(DiamondBrush),
}

impl Default for Brush {
    fn default() -> Self {
        DiamondBrush::default().into()
    }
}

impl BrushAction for Brush {
    fn apply(&self, center: DrawingBundle) -> Vec<DrawingBundle> {
        match self {
            Brush::SingleTile => SingleTileBrush.apply(center),
            Brush::Round(brush) => brush.apply(center),
            Brush::Square(brush) => brush.apply(center),
            Brush::Diamond(brush) => brush.apply(center),
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Reflect, Copy, Clone, Hash)]
pub struct SingleTileBrush;
impl BrushAction for SingleTileBrush {
    fn apply(&self, center: DrawingBundle) -> Vec<DrawingBundle> {
        vec![center]
    }
}
