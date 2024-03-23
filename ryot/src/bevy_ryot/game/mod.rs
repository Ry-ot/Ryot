use core::fmt;
use std::marker::PhantomData;

use crate::prelude::*;
use bevy::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use self::{elevation::Elevation, map::MapTiles, sprites::SPRITE_BASE_SIZE};

pub mod elevation;

pub struct GamePlugin<C: AppearanceAssets>(PhantomData<C>);

impl<C: AppearanceAssets> GamePlugin<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C: AppearanceAssets> Default for GamePlugin<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: AppearanceAssets> Plugin for GamePlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadObjects>()
            .add_systems(Update, apply_elevation::<C>);
    }
}

#[derive(
    Component,
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    Default,
    Reflect,
    PartialOrd,
    Ord,
)]
pub enum GameObjectId {
    #[default]
    None,
    Object(u32),
    Outfit(u32),
    Effect(u32),
    Missile(u32),
}

impl GameObjectId {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn as_group_and_id(&self) -> Option<(AppearanceGroup, u32)> {
        match self {
            GameObjectId::None => None,
            GameObjectId::Object(id) => Some((AppearanceGroup::Object, *id)),
            GameObjectId::Outfit(id) => Some((AppearanceGroup::Outfit, *id)),
            GameObjectId::Effect(id) => Some((AppearanceGroup::Effect, *id)),
            GameObjectId::Missile(id) => Some((AppearanceGroup::Missile, *id)),
        }
    }

    pub fn group(&self) -> Option<AppearanceGroup> {
        self.as_group_and_id().map(|(group, _)| group)
    }

    pub fn id(&self) -> Option<u32> {
        self.as_group_and_id().map(|(_, id)| id)
    }
}

impl fmt::Display for GameObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameObjectId::None => write!(f, "None"),
            GameObjectId::Object(id) => write!(f, "Object({})", id),
            GameObjectId::Outfit(id) => write!(f, "Outfit({})", id),
            GameObjectId::Effect(id) => write!(f, "Effect({})", id),
            GameObjectId::Missile(id) => write!(f, "Missile({})", id),
        }
    }
}

#[derive(Bundle, Copy, Clone, Debug)]
pub struct GameObjectBundle {
    pub object_id: GameObjectId,
    pub position: TilePosition,
    pub layer: Layer,
}

impl GameObjectBundle {
    pub fn new(object_id: GameObjectId, position: TilePosition, layer: Layer) -> Self {
        Self {
            object_id,
            position,
            layer,
        }
    }
}

#[derive(Event, Clone, Debug, Deref, DerefMut)]
pub struct LoadObjects(pub Vec<GameObjectBundle>);

type ElevationFilter = (
    With<GameObjectId>,
    Or<(Changed<Visibility>, Changed<TilePosition>)>,
);

fn apply_elevation<C: AppearanceAssets>(
    appearance_asets: Res<C>,
    q_tile: Query<(&TilePosition, &Layer), ElevationFilter>,
    mut q_entities: Query<(&mut Elevation, &Visibility, &GameObjectId)>,
    map_tiles: Res<MapTiles<Entity>>,
) {
    let appearances = appearance_asets.prepared_appearances();
    for tile in q_tile
        .iter()
        .filter(|(_, layer)| matches!(layer, Layer::Bottom(_)))
        .map(|(pos, _)| *pos)
        .unique()
        .filter_map(|pos| map_tiles.get(&pos))
    {
        tile.into_iter()
            .filter(|(layer, _)| matches!(layer, Layer::Bottom(_)))
            .map(|(_, entity)| entity)
            .fold(0., |tile_elevation, entity| {
                let Ok((mut elevation, visibility, object_id)) = q_entities.get_mut(entity) else {
                    return tile_elevation;
                };
                let Some((group, id)) = object_id.as_group_and_id() else {
                    return tile_elevation;
                };

                let appearance = appearances.get_for_group(group, id).cloned();
                let elevation_delta = appearance
                    .and_then(|app| app.flags?.height?.elevation)
                    .unwrap_or(0) as f32
                    / SPRITE_BASE_SIZE.y as f32;
                elevation.elevation = match group {
                    AppearanceGroup::Object => tile_elevation,
                    AppearanceGroup::Outfit => tile_elevation,
                    AppearanceGroup::Effect => 0.,
                    AppearanceGroup::Missile => 0.,
                };

                if visibility != Visibility::Hidden {
                    tile_elevation + elevation_delta
                } else {
                    tile_elevation
                }
            });
    }
}
