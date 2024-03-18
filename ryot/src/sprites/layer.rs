use crate::position::TilePosition;
#[cfg(feature = "bevy")]
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    ops::Div,
};
use strum::*;

/// A type that represents the order of an item in an ordered layer. There is a virtual limit of
/// 256 items in the ordered layers.
pub type Order = u8;

const MAX_Z: f32 = 999.;
const MAX_Z_TILE: f32 = i8::MAX as f32;
const MAX_Z_TRANSFORM: f32 = 900. - MAX_Z_TILE;
const COUNT_LAYERS: f32 = Layer::COUNT as f32;
const LAYER_WIDTH: f32 = MAX_Z_TRANSFORM / (2. * i16::MAX as f32);

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

impl Display for Layer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ground => write!(f, "G"),
            Self::Edge => write!(f, "E"),
            Self::Bottom(bottom_layer) => write!(f, "B({})", bottom_layer),
            Self::Top => write!(f, "T"),
            Self::Hud(order) => write!(f, "H({})", order),
        }
    }
}

impl Display for BottomLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.relative_layer, self.order)
    }
}

impl Display for RelativeLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Object => write!(f, "O"),
            Self::Creature => write!(f, "C"),
            Self::Effect => write!(f, "E"),
            Self::Missile => write!(f, "M"),
        }
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Layer {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        match usize::arbitrary(g) % Self::COUNT {
            0 => Self::Ground,
            1 => Self::Edge,
            2 => Self::Bottom(BottomLayer::arbitrary(g)),
            3 => Self::Top,
            4 => Self::Hud(Order::arbitrary(g) % 100),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for BottomLayer {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Self {
            relative_layer: RelativeLayer::arbitrary(g),
            order: Order::arbitrary(g) % BottomLayer::TOP_MOST_LAYER,
        }
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for RelativeLayer {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        match usize::arbitrary(g) % Self::COUNT {
            0 => Self::Object,
            1 => Self::Creature,
            2 => Self::Effect,
            3 => Self::Missile,
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "debug")]
impl From<&Layer> for Color {
    fn from(value: &Layer) -> Self {
        match value {
            Layer::Ground => Color::ORANGE,
            Layer::Edge => Color::YELLOW,
            Layer::Bottom(layer) => Color::from(layer.relative_layer),
            Layer::Top => Color::PINK,
            Layer::Hud(_) => Color::TURQUOISE,
        }
    }
}

#[cfg(feature = "debug")]
impl From<RelativeLayer> for Color {
    fn from(value: RelativeLayer) -> Self {
        match value {
            RelativeLayer::Object => Color::RED,
            RelativeLayer::Creature => Color::GREEN,
            RelativeLayer::Effect => Color::BLUE,
            RelativeLayer::Missile => Color::ALICE_BLUE,
        }
    }
}

impl Iterator for Layer {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Self::Ground => Some(Self::Edge),
            Self::Edge => Some(Self::Bottom(Default::default())),
            Self::Bottom(mut bottom_layer) => bottom_layer
                .relative_layer
                .next()
                .map(|relative_layer| {
                    Self::Bottom(BottomLayer {
                        order: 0,
                        relative_layer,
                    })
                })
                .or(Some(Self::Top)),
            Self::Top => Some(Self::Hud(0)),
            Self::Hud(order) => {
                if order < Order::MAX {
                    Some(Self::Hud(order + 1))
                } else {
                    None
                }
            }
        }
    }
}

impl DoubleEndedIterator for Layer {
    fn next_back(&mut self) -> Option<Self::Item> {
        match *self {
            Self::Ground => None,
            Self::Edge => Some(Self::Ground),
            Self::Bottom(mut bottom_layer) => bottom_layer
                .relative_layer
                .next_back()
                .map(|relative_layer| {
                    Self::Bottom(BottomLayer {
                        order: BottomLayer::TOP_MOST_LAYER,
                        relative_layer,
                    })
                })
                .or(Some(Self::Edge)),
            Self::Top => Some(Self::Bottom(BottomLayer {
                order: BottomLayer::TOP_MOST_LAYER,
                relative_layer: RelativeLayer::iter().last().unwrap(),
            })),
            Self::Hud(order) => {
                if order == 0 {
                    Some(Self::Top)
                } else {
                    Some(Self::Hud(order - 1))
                }
            }
        }
    }
}

impl Layer {
    pub fn z(&self) -> f32 {
        match *self {
            Self::Ground => 0. * LAYER_WIDTH,
            Self::Edge => 1. * LAYER_WIDTH,
            Self::Bottom(layer) => 2. * LAYER_WIDTH + layer.z(),
            Self::Top => 3. * LAYER_WIDTH,
            Self::Hud(order) => (MAX_Z_TRANSFORM + 2. + order as f32).min(MAX_Z),
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

impl Iterator for RelativeLayer {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Self::Object => Some(Self::Creature),
            Self::Creature => Some(Self::Effect),
            Self::Effect => Some(Self::Missile),
            Self::Missile => None,
        }
    }
}

impl DoubleEndedIterator for RelativeLayer {
    fn next_back(&mut self) -> Option<Self::Item> {
        match *self {
            Self::Object => None,
            Self::Creature => Some(Self::Object),
            Self::Effect => Some(Self::Creature),
            Self::Missile => Some(Self::Effect),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct BottomLayer {
    pub order: Order,
    pub relative_layer: RelativeLayer,
}

impl Default for BottomLayer {
    fn default() -> Self {
        Self {
            order: Order::MAX,
            relative_layer: Default::default(),
        }
    }
}

impl Iterator for BottomLayer {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if self.order < BottomLayer::TOP_MOST_LAYER {
            Some(Self {
                order: self.order + 1,
                relative_layer: self.relative_layer,
            })
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for BottomLayer {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.order > 0 {
            Some(Self {
                order: self.order - 1,
                relative_layer: self.relative_layer,
            })
        } else {
            None
        }
    }
}

impl BottomLayer {
    const TOP_MOST_LAYER: u8 = 10;
    const COUNT_RELATIVE_LAYERS: f32 = RelativeLayer::COUNT as f32;
    const RELATIVE_WIDTH: f32 = LAYER_WIDTH / COUNT_LAYERS;

    pub fn new(order: Order, relative_layer: RelativeLayer) -> Self {
        Self {
            order,
            relative_layer,
        }
    }

    pub fn stack(relative_layer: RelativeLayer) -> Self {
        Self {
            order: Order::MAX,
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
        let order_width = width / BottomLayer::TOP_MOST_LAYER as f32;
        // Our order relative to other elements of the same layer in our stack weighed against our width window
        let order = self.order as f32 * order_width;
        // Final number between 0.0..TOTAL_WIDTH
        min + order
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
    if matches!(layer, Layer::Hud(_)) {
        return layer.z();
    }

    let weight = u16::MAX as f32;
    let x_weighted = MAX_Z_TRANSFORM.div(2.) * pos.x as f32 / weight;
    let y_weighted = MAX_Z_TRANSFORM * pos.y as f32 / weight;

    pos.z as f32 + x_weighted - y_weighted + layer.z()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::reflect::Enum;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn compute_z_transform_layer_order(pos: TilePosition, layer1: Layer, layer2: Layer) -> bool {
        let z1 = compute_z_transform(&pos, &layer1);
        let z2 = compute_z_transform(&pos, &layer2);

        let (result, comparison) = match (layer1, layer2) {
            (Layer::Hud(o1), Layer::Hud(o2)) if o1 > o2 => (z1 > z2, "z1 > z2"),
            (Layer::Hud(o1), Layer::Hud(o2)) if o1 <= o2 => (z1 <= z2, "z1 <= z2"),
            (Layer::Hud(_), _) => (z1 > z2, "z1 > z2"),
            (_, Layer::Hud(_)) => (z1 < z2, "z1 < z2"),
            (Layer::Top, Layer::Top) => (z1 == z2, "z1 == z2"),
            (Layer::Top, _) => (z1 > z2, "z1 > z2"),
            (_, Layer::Top) => (z1 < z2, "z1 < z2"),
            (Layer::Bottom(b1), Layer::Bottom(b2)) => {
                if b1.relative_layer == b2.relative_layer {
                    if b1.order > b2.order {
                        (z1 > z2, "z1 > z2")
                    } else {
                        (z1 <= z2, "z1 <= z2")
                    }
                } else if b1.relative_layer.variant_index() <= b2.relative_layer.variant_index() {
                    (z1 < z2, "z1 < z2")
                } else {
                    (z1 > z2, "z1 > z2")
                }
            }
            (Layer::Bottom(_), _) => (z1 > z2, "z1 > z2"),
            (_, Layer::Bottom(_)) => (z1 < z2, "z1 < z2"),
            (Layer::Edge, Layer::Edge) => (z1 == z2, "z1 == z2"),
            (Layer::Edge, _) => (z1 > z2, "z1 > z2"),
            (_, Layer::Edge) => (z1 < z2, "z1 < z2"),
            (Layer::Ground, Layer::Ground) => (z1 == z2, "z1 == z2"),
        };
        if !result {
            println!("Failed:");
            println!("\t     pos: {:?}", pos);
            println!("\t  layer1: {:?}", layer1);
            println!("\t       z: {}", z1);
            println!("\t  layer2: {:?}", layer2);
            println!("\t       z: {}", z2);
            println!("\texpected:{}", comparison);
        }
        result
    }

    #[quickcheck]
    fn compute_z_transform_45deg_rules(pos: TilePosition, layer1: Layer, layer2: Layer) -> bool {
        let east = pos + TilePosition::new(1, 0, 0);
        let south = pos + TilePosition::new(0, -1, 0);
        let south_east = pos + TilePosition::new(1, -1, 0);
        let z = compute_z_transform(&pos, &layer1);
        let east_z = compute_z_transform(&east, &layer2);
        let south_z = compute_z_transform(&south, &layer2);
        let south_east_z = compute_z_transform(&south_east, &layer2);
        let mut message = "".to_owned();
        let mut result = true;

        if east_z <= z {
            message.push_str("\t\teast_z <= z\n");
            result = false;
        }
        if south_z <= z {
            message.push_str("\t\tsouth_z <= z\n");
            result = false;
        }
        if south_east_z <= z {
            message.push_str("\t\tsouth_east_z <= z\n");
            result = false;
        }
        if south_east_z <= east_z {
            message.push_str("\t\tsouth_east_z <= east_z\n");
            result = false;
        }
        if south_east_z <= south_z {
            message.push_str("\t\tsouth_east_z <= south_z\n");
            result = false;
        }

        let result = match (layer1, layer2) {
            (Layer::Ground, Layer::Ground) => result,
            (Layer::Edge, Layer::Edge) => result,
            (Layer::Bottom(_), Layer::Bottom(_)) => result,
            (Layer::Top, Layer::Top) => result,
            _ => true, // We don't care about the other cases, this is handled by the other tests
        };

        if !result {
            println!("Failed:");
            println!("\t      pos: {:?}", pos);
            println!("\t   layer1: {:?}", layer1);
            println!("\t        z: {}", z);
            println!("\t   layer2: {:?}", layer2);
            println!("\t    east-z: {}", east_z);
            println!("\t   south-z: {}", south_z);
            println!("\tsouth-east-z: {}", south_east_z);
            println!("{}", message);
        }

        true
    }
}
