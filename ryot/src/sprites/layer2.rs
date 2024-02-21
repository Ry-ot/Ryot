use crate::position::TilePosition;
use bevy::prelude::*;
use strum::*;

/// A type that represents the order of an item in an ordered layer. There is a virtual limit of
/// 256 items in the ordered layers.
type Order = u8;

/// A type that represents the id of an object in the game. There is a virtual limit of u32::MAX
/// for the number of objects in the game.
type ObjectId = u32;

const MAX_Z: f32 = i8::MAX as f32;
const MAX_Z_TRANSFORM: f32 = 900. - MAX_Z;
const COUNT_LAYERS: f32 = Layer2::COUNT as f32;
const LAYER_WIDTH: f32 = MAX_Z_TRANSFORM / (2. * u16::MAX as f32);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumCount)]
pub enum Layer2 {
    #[default]
    Ground,
    Edge,
    Bottom(BottomLayer),
    Top,
}

impl Layer2 {
    fn z(&self) -> f32 {
        match *self {
            Self::Ground => 0. * LAYER_WIDTH,
            Self::Edge => 1. * LAYER_WIDTH,
            Self::Bottom(layer) => 2. * LAYER_WIDTH + layer.z(),
            Self::Top => MAX_Z_TRANSFORM + 1.,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumCount)]
enum RelativeLayer {
    #[default]
    Object,
    Creature,
    Effect,
    Missile,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BottomLayer {
    order: Order,
    relative_layer: RelativeLayer,
}

impl BottomLayer {
    const COUNT_RELATIVE_LAYERS: f32 = RelativeLayer::COUNT as f32;
    const RELATIVE_WIDTH: f32 = LAYER_WIDTH / COUNT_LAYERS;

    pub(crate) fn z(&self) -> f32 {
        // Our layer number relative to others in the stack
        let val = (self.relative_layer as usize) as f32;
        // How much space we can use of the f32 value from 0.0..RELATIVE_WIDTH
        let width = Self::RELATIVE_WIDTH / Self::COUNT_RELATIVE_LAYERS;
        // Where our range starts
        let min = val * width;
        // Our order relative to other elements of the same layer in our stack weighed against our width window
        let weight = self.order as f32 / Order::MAX as f32;
        // Final number between 0.0..TOTAL_WIDTH
        min + weight / Self::COUNT_RELATIVE_LAYERS
    }
}

fn compute_z_transform(pos: &TilePosition, layer: &Layer2) -> f32 {
    let weight = u16::MAX as f32;
    let xy_weighed =
        (MAX_Z_TRANSFORM * pos.x as f32 / weight) - (MAX_Z_TRANSFORM * pos.y as f32 / weight);
    pos.z as f32 + xy_weighed + layer.z()
}
