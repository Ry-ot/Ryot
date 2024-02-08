use crate::{Brush, BrushAction, DiamondBrush, RoundBrush, SquareBrush};
use bevy::prelude::Reflect;
use ryot::bevy_ryot::drawing::DrawingBundle;

#[derive(Debug, Eq, PartialEq, Reflect, Copy, Clone, Hash)]
pub enum GeometricBrush {
    Round(i32),
    Square(i32),
    Diamond(i32),
}

impl From<GeometricBrush> for Brush {
    fn from(brush: GeometricBrush) -> Self {
        Brush::Geometric(brush)
    }
}

impl GeometricBrush {
    pub fn get_size(&self) -> i32 {
        match self {
            GeometricBrush::Round(size) => *size,
            GeometricBrush::Square(size) => *size,
            GeometricBrush::Diamond(size) => *size,
        }
    }

    pub fn set_size(&mut self, size: i32) {
        let new_size = size.abs().clamp(1, 50);

        match self {
            GeometricBrush::Round(size) => *size = new_size,
            GeometricBrush::Square(size) => *size = new_size,
            GeometricBrush::Diamond(size) => *size = new_size,
        }
    }

    pub fn increase(&mut self) {
        self.set_size(self.get_size() + 1);
    }

    pub fn decrease(&mut self) {
        self.set_size(self.get_size() - 1);
    }
}

impl BrushAction for GeometricBrush {
    fn apply(&self, center: DrawingBundle) -> Vec<DrawingBundle> {
        match self {
            GeometricBrush::Round(size) => RoundBrush(*size).apply(center),
            GeometricBrush::Square(size) => SquareBrush(*size).apply(center),
            GeometricBrush::Diamond(size) => DiamondBrush(*size).apply(center),
        }
    }
}
