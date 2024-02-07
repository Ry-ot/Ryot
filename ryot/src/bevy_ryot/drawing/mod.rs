use crate::appearances::{self, FixedFrameGroup};
use crate::bevy_ryot::AppearanceDescriptor;
use crate::position::TilePosition;
use bevy::prelude::*;
use bevy::utils::HashMap;

mod commands;
pub use commands::*;

#[derive(Debug, Default, Resource, Deref, Reflect, DerefMut)]
pub struct MapTiles(pub HashMap<TilePosition, HashMap<Layer, Entity>>);

#[derive(Bundle, Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct DrawingBundle {
    pub layer: Layer,
    pub tile_pos: TilePosition,
    pub appearance: AppearanceDescriptor,
    pub visibility: Visibility,
}

impl DrawingBundle {
    pub fn new(layer: Layer, tile_pos: TilePosition, appearance: AppearanceDescriptor) -> Self {
        Self {
            layer,
            tile_pos,
            appearance,
            visibility: Visibility::default(),
        }
    }

    pub fn object(layer: Layer, tile_pos: TilePosition, id: u32) -> Self {
        Self::new(layer, tile_pos, AppearanceDescriptor::object(id))
    }

    pub fn creature(tile_pos: TilePosition, id: u32, frame_group_index: FixedFrameGroup) -> Self {
        Self::new(
            Layer::Creature,
            tile_pos,
            AppearanceDescriptor::outfit(id, frame_group_index),
        )
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn with_layer(mut self, layer: Layer) -> Self {
        self.layer = layer;
        self
    }
}

/*
Drawing levels (keeping it around 100k sprites per level):
- Max details: 1 floor, 1 top, 1 bottom, 1 ground and 10 contents - ~64x64
- Medium details: 1 floor: 1 top, 1 bottom, 1 ground and 5 content - ~112x112
- Minimal details: 1 floor: 1 top, 1 bottom, 1 ground and 1 content - ~160x160
- Ground+top: 1 floor, 1 top, 1 ground - 224x224
- Ground only - 320x320
- >320x320 - Not possible (maybe chunk view so that people can navigate through the map quicker in the future)
- Draw rules change per detail level

We load 2-3x the current view but we only set as visible the 1.1 * view.
As we move the camera, we set the new tiles as visible and the old ones as hidden and we deload/load the edges (as hidden)
As we zoom in and out, we change the detail level of the tiles and change visibility accordingly.

So when a click happens the first tihng that it does is a c
*/

/// This enum defines the layers that composes a game.
/// The base layers are defined in the enum and custom layers can be added.
/// The layers are used to define the rendering order of the tiles.
/// There are two types of layers: dynamic and static.
///     - Dynamic layers are used for elements that can change their rendering order
///     depending on their attributes like position or floor. E.g.: creatures, items, effects.
///    - Static layers are used for elements that have a fixed rendering order and are always
///    rendered on top of dynamic layers. E.g.: mouse, grid, ui elements.
///
/// Static layers are separated from dynamic layers by the StaticLowerBound layer.
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone, Default, Reflect, Component)]
pub enum Layer {
    #[default]
    Items,
    Top,
    Bottom,
    Ground,
    Creature,
    Effect,
    StaticLowerBound,
    Cursor,
    Custom(i32),
}

impl Layer {
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
            Self::Top => 90,
            Self::Bottom => 80,
            Self::Effect => 60,
            Self::Creature => 40,
            Self::Items => 20,
            Self::Ground => 0,
            Self::StaticLowerBound => 900,
            Self::Cursor => 999,
            Self::Custom(z) => *z,
        }
    }
}

impl From<Option<appearances::AppearanceFlags>> for Layer {
    fn from(flags: Option<appearances::AppearanceFlags>) -> Self {
        match flags {
            Some(flags) if flags.top.is_some() => Self::Top,
            Some(flags) if flags.bottom.is_some() => Self::Bottom,
            Some(flags) if flags.bank.is_some() || flags.clip.is_some() => Self::Ground,
            _ => Self::Items,
        }
    }
}
