use crate::position::TilePosition;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use strum::*;

/// A type that represents the order of an item in an ordered layer. There is a virtual limit of
/// 256 items in the ordered layers.
type Order = u8;

const MAX_Z: f32 = 999.;
const MAX_Z_TILE: f32 = i8::MAX as f32;
const MAX_Z_TRANSFORM: f32 = 900. - MAX_Z_TILE;
const COUNT_LAYERS: f32 = Layer::COUNT as f32;
const LAYER_WIDTH: f32 = MAX_Z_TRANSFORM / (2. * u16::MAX as f32);

#[derive(
    Debug,
    Default,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumIter,
    EnumCount,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
pub enum Layer {
    #[default]
    Ground,
    Edge,
    Bottom(BottomLayer),
    Top,
    Hud(Order),
}

impl Layer {
    pub fn z(&self) -> f32 {
        match *self {
            Self::Ground => 0. * LAYER_WIDTH,
            Self::Edge => 1. * LAYER_WIDTH,
            Self::Bottom(layer) => 2. * LAYER_WIDTH + layer.z(),
            Self::Top => MAX_Z_TRANSFORM + 1.,
            Self::Hud(order) => (MAX_Z - order as f32).max(MAX_Z_TRANSFORM + 2.),
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumCount, Serialize, Deserialize,
)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub enum RelativeLayer {
    #[default]
    Object,
    Creature,
    Effect,
    Missile,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct BottomLayer {
    order: Order,
    relative_layer: RelativeLayer,
}

impl BottomLayer {
    const COUNT_RELATIVE_LAYERS: f32 = RelativeLayer::COUNT as f32;
    const RELATIVE_WIDTH: f32 = LAYER_WIDTH / COUNT_LAYERS;

    pub fn new(order: Order, relative_layer: RelativeLayer) -> Self {
        Self {
            order,
            relative_layer,
        }
    }

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

impl PartialOrd for BottomLayer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BottomLayer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z().partial_cmp(&other.z()).unwrap()
    }
}

pub fn compute_z_transform(pos: &TilePosition, layer: &Layer) -> f32 {
    let weight = u16::MAX as f32;
    let xy_weighed =
        (MAX_Z_TRANSFORM * pos.x as f32 / weight) - (MAX_Z_TRANSFORM * pos.y as f32 / weight);
    pos.z as f32 + xy_weighed + layer.z()
}
