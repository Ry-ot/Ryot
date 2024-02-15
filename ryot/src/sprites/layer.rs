use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// This enum defines the layers that composes a game.
/// The base layers are defined in the enum and custom layers can be added.
/// The layers are used to define the rendering order of the tiles.
/// There are two types of layers: dynamic and static.
///    - TopDown45 layers are used for elements that have their z displacement based on its
///    position in the grid.
///    - Fixed layers are used for elements that have a fixed rendering order and are always
///    rendered on top of dynamic layers.
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
pub enum Layer {
    Fixed(i32),
    TopDown45(i32),
}

impl Default for Layer {
    fn default() -> Self {
        CipLayer::Items.into()
    }
}

impl Layer {
    pub const TOP_MOST_LAYER: Layer = Layer::Fixed(999);
    pub const BOTTOM_MOST_LAYER: Layer = Layer::Fixed(-999);

    /// Z is calculated based on floor position and layer.
    /// The base Z is floor * 100 and the layer adds an offset.
    /// The offset is used to calculate the rendering order of the tile.
    /// Tiles with higher Z are rendered on top of tiles with lower Z.
    /// The tile Z for the game is always the floor (Z / 100).floor().
    ///
    /// We leave a gap of 10 between layers to allow for more layers to be added
    /// in the future and to make it possible to place custom layers between
    /// the default ones.
    pub fn z(&self) -> i32 {
        match self {
            Self::Fixed(z) => *z,
            Self::TopDown45(z) => *z,
        }
    }
}

#[derive(Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Resource))]
pub struct Layers(pub Vec<Layer>);

impl Layers {
    pub fn get_sorted_by_z_desc(&self) -> Vec<&Layer> {
        let mut layers: Vec<_> = self.iter().collect();
        layers.sort_by_key(|b| std::cmp::Reverse(b.z()));
        layers
    }
}

impl Default for Layers {
    fn default() -> Self {
        Self(vec![
            CipLayer::Missile.into(),
            CipLayer::Top.into(),
            CipLayer::Bottom.into(),
            CipLayer::Effect.into(),
            CipLayer::Creature.into(),
            CipLayer::Items.into(),
            CipLayer::Ground.into(),
        ])
    }
}

#[derive(Hash, Eq, Default, PartialEq, Debug, Copy, Clone)]
pub enum CipLayer {
    Missile,
    Top,
    Bottom,
    Effect,
    Creature,
    #[default]
    Items,
    Ground,
}

impl From<CipLayer> for Layer {
    fn from(layer: CipLayer) -> Self {
        match layer {
            CipLayer::Missile => Layer::TopDown45(100),
            CipLayer::Top => Layer::TopDown45(90),
            CipLayer::Bottom => Layer::TopDown45(80),
            CipLayer::Effect => Layer::TopDown45(60),
            CipLayer::Creature => Layer::TopDown45(40),
            CipLayer::Items => Layer::TopDown45(20),
            CipLayer::Ground => Layer::TopDown45(0),
        }
    }
}

impl From<Layer> for CipLayer {
    fn from(layer: Layer) -> Self {
        match layer {
            Layer::TopDown45(100) => CipLayer::Missile,
            Layer::TopDown45(90) => CipLayer::Top,
            Layer::TopDown45(80) => CipLayer::Bottom,
            Layer::TopDown45(60) => CipLayer::Effect,
            Layer::TopDown45(40) => CipLayer::Creature,
            Layer::TopDown45(20) => CipLayer::Items,
            Layer::TopDown45(0) => CipLayer::Ground,
            _ => CipLayer::Items,
        }
    }
}
