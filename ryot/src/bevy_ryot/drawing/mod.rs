use crate::appearances::{self, FixedFrameGroup};
use crate::bevy_ryot::{AppearanceDescriptor, InternalContentState};
use crate::position::TilePosition;
use bevy::prelude::*;
use bevy::utils::HashMap;

mod commands;
pub use commands::*;

pub struct DrawingPlugin;

impl Plugin for DrawingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Layer>()
            .register_type::<DetailLevel>()
            .add_systems(
                Update,
                (reduce_detail_level, increase_detail_level)
                    .chain()
                    .run_if(in_state(InternalContentState::Ready)),
            );
    }
}

/// A resource that holds the map tiles and the entities that are drawn on them.
/// An entity location is represented by the combination of a Layer and a Position.
/// The MapTiles are represented by a HashMap of TilePosition and a HashMap of Layer and Entity.
/// The MapTiles is used to keep track of the entities that are drawn on the map and their position.
#[derive(Debug, Default, Resource, Deref, Reflect, DerefMut)]
pub struct MapTiles(pub HashMap<TilePosition, HashMap<Layer, Entity>>);

#[derive(Component, Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Tile;

/// A bundle that represents an entity drawn to a location (Layer + TilePosition) in the map.
/// The DrawingBundle is used to create and update the entities that are drawn on the map.
/// It holds the layer, the tile position, the appearance descriptor and the visibility of the entity.
///
/// Visibility is mainly a sprite component, however, for performance reasons, we use the sprite
/// visibility to control the visibility of the tile content. This way, we can reduce the amount of
/// effort made to drawn the map, by not drawing the tiles that are not visible, while still keeping
/// them as entity.
#[derive(Bundle, Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct DrawingBundle {
    pub layer: Layer,
    pub tile_pos: TilePosition,
    pub appearance: AppearanceDescriptor,
    pub visibility: Visibility,
    pub tile: Tile,
}

impl DrawingBundle {
    pub fn new(layer: Layer, tile_pos: TilePosition, appearance: AppearanceDescriptor) -> Self {
        Self {
            layer,
            tile_pos,
            appearance,
            tile: Tile,
            visibility: Visibility::Visible,
        }
    }

    pub fn from_tile_position(tile_pos: TilePosition) -> Self {
        Self {
            tile_pos,
            ..Default::default()
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

#[derive(Debug, Copy, Clone, Reflect, Component)]
pub struct WontDraw;

/// Drawing levels (we try to keep a maximum of 100k visible sprites per level):
///     - Max details: 1 floor, 1 top, 1 bottom, 1 ground and 10 contents - ~64x64
///     - Medium details: 1 floor: 1 top, 1 bottom, 1 ground and 5 content - ~112x112
///     - Minimal details: 1 floor: 1 top, 1 bottom, 1 ground and 1 content - ~160x160
///     - Ground+bottom: 1 floor, 1 bottom, 1 ground - 224x224
///     - Ground only - 320x320
///     - >320x320 - Not possible (maybe chunk view so that people can navigate through the map quicker in the future)
///     - Draw rules change per detail level
///
/// We load 2-3x the current view but we only set as visible the 1.1 * view.
/// As we move the camera, we set the new tiles as visible and the old ones as hidden and we deload/load the edges (as hidden)
/// As we zoom in and out, we change the detail level of the tiles and change visibility accordingly.
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone, Default, Reflect, Component)]
pub enum DetailLevel {
    #[default]
    Max,
    Medium,
    Minimal,
    GroundBottom,
    GroundOnly,
    None,
}

impl DetailLevel {
    pub fn from_area(area: u32) -> Self {
        let size = (area as f32).sqrt() as i32;
        match size {
            0..=64 => Self::Max,
            65..=112 => Self::Medium,
            113..=160 => Self::Minimal,
            161..=224 => Self::GroundBottom,
            225..=320 => Self::GroundOnly,
            _ => Self::None,
        }
    }

    pub fn visible_layers(&self) -> Vec<Layer> {
        match self {
            Self::Max => vec![
                Layer::Items,
                Layer::Top,
                Layer::Bottom,
                Layer::Ground,
                Layer::Creature,
                Layer::Effect,
            ],
            Self::Medium => vec![Layer::Items, Layer::Top, Layer::Bottom, Layer::Ground],
            Self::Minimal => vec![Layer::Items, Layer::Bottom, Layer::Ground],
            Self::GroundBottom => vec![Layer::Bottom, Layer::Ground],
            Self::GroundOnly => vec![Layer::Ground],
            Self::None => vec![],
        }
    }
}

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

fn reduce_detail_level(
    mut commands: Commands,
    camera_query: Query<&DetailLevel, (With<Camera>, Changed<DetailLevel>)>,
    mut visible_entities: Query<(Entity, &mut Visibility, &Layer), (Without<WontDraw>, With<Tile>)>,
) {
    for detail_level in camera_query.iter() {
        for (entity, mut visibility, layer) in visible_entities.iter_mut() {
            if *visibility == Visibility::Hidden {
                continue;
            }

            if !detail_level.visible_layers().contains(layer) {
                *visibility = Visibility::Hidden;
                commands.entity(entity).insert(WontDraw);
            }
        }
    }
}

fn increase_detail_level(
    mut commands: Commands,
    camera_query: Query<&DetailLevel, (With<Camera>, Changed<DetailLevel>)>,
    mut invisible_entities: Query<(Entity, &mut Visibility, &Layer), (With<WontDraw>, With<Tile>)>,
) {
    for detail_level in camera_query.iter() {
        for (entity, mut visibility, layer) in invisible_entities.iter_mut() {
            if *visibility == Visibility::Visible {
                continue;
            }

            if detail_level.visible_layers().contains(layer) {
                *visibility = Visibility::Visible;
                commands.entity(entity).remove::<WontDraw>();
            }
        }
    }
}
