use crate::appearances::{self, FixedFrameGroup};
use crate::bevy_ryot::{AppearanceDescriptor, InternalContentState};
use crate::layer::*;
use crate::position::TilePosition;
use bevy::prelude::*;

mod brushes;
pub use brushes::*;

mod commands;
pub use commands::*;

pub struct DrawingPlugin;

impl Plugin for DrawingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Layers>()
            .register_type::<Layer>()
            .register_type::<DetailLevel>()
            .add_systems(
                Update,
                (reduce_detail_level, increase_detail_level)
                    .chain()
                    .run_if(in_state(InternalContentState::Ready)),
            );
    }
}

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

impl BrushItem for DrawingBundle {
    fn from_position(original: Self, tile_pos: TilePosition) -> Self {
        let DrawingBundle {
            layer,
            appearance,
            visibility,
            tile,
            ..
        } = original;

        DrawingBundle {
            layer,
            tile_pos,
            appearance,
            visibility,
            tile,
        }
    }

    fn get_position(&self) -> TilePosition {
        self.tile_pos
    }
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
            CipLayer::Creature.into(),
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
            Self::Max => Layers::default().0,
            Self::Medium => vec![
                CipLayer::Items.into(),
                CipLayer::Top.into(),
                CipLayer::Bottom.into(),
                CipLayer::Ground.into(),
            ],
            Self::Minimal => vec![
                CipLayer::Items.into(),
                CipLayer::Bottom.into(),
                CipLayer::Ground.into(),
            ],
            Self::GroundBottom => vec![CipLayer::Bottom.into(), CipLayer::Ground.into()],
            Self::GroundOnly => vec![CipLayer::Ground.into()],
            Self::None => vec![],
        }
    }
}

impl From<Option<appearances::AppearanceFlags>> for Layer {
    fn from(flags: Option<appearances::AppearanceFlags>) -> Self {
        match flags {
            Some(flags) if flags.top.is_some() => CipLayer::Top.into(),
            Some(flags) if flags.bottom.is_some() => CipLayer::Bottom.into(),
            Some(flags) if flags.bank.is_some() || flags.clip.is_some() => CipLayer::Ground.into(),
            _ => Self::default(),
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
