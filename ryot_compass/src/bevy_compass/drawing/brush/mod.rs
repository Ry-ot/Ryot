use bevy::prelude::*;
use ryot::prelude::drawing::*;

mod diamond;
pub use diamond::DiamondBrush;

mod round;
pub use round::RoundBrush;
use ryot::bevy_ryot::AppearanceDescriptor;
use ryot::position::TilePosition;

mod square;
pub use square::SquareBrush;

mod systems;
pub use systems::update_brush;

pub trait BrushAction {
    fn apply(&self, center: DrawingBundle) -> Vec<DrawingBundle>;
    fn get_positions(&self, center: TilePosition) -> Vec<TilePosition> {
        self.apply(DrawingBundle::new(
            Layer::Ground,
            center,
            AppearanceDescriptor::default(),
        ))
        .into_iter()
        .map(|bundle| bundle.tile_pos)
        .collect()
    }
}

#[derive(Debug, Eq, Default, PartialEq, Reflect, Copy, Clone, Hash)]
pub enum BrushType {
    Round,
    Square,
    #[default]
    Diamond,
}

#[derive(Component, Default, Eq, PartialEq, Reflect, Copy, Clone, Hash)]
pub struct Brush {
    pub size: i32,
    pub brush_type: BrushType,
}

impl Brush {
    pub fn new(size: i32, brush_type: BrushType) -> Self {
        Brush { size, brush_type }
    }
}

impl BrushAction for Brush {
    fn apply(&self, center: DrawingBundle) -> Vec<DrawingBundle> {
        match self.brush_type {
            BrushType::Round => RoundBrush(self.size).apply(center),
            BrushType::Square => SquareBrush(self.size).apply(center),
            BrushType::Diamond => DiamondBrush(self.size).apply(center),
        }
    }
}
