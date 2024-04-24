use crate::bevy_ryot::sprites::SPRITE_BASE_SIZE;
use bevy::prelude::*;
use itertools::Itertools;
use ryot_content::prelude::{EntityType, GameObjectId, VisualElements};
use ryot_tiled::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Component, Copy, PartialEq, Serialize, Deserialize)]
pub struct Elevation {
    pub elevation: f32,
}

impl Default for Elevation {
    fn default() -> Self {
        Elevation { elevation: 0.0 }
    }
}

impl Display for Elevation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E:{}", self.elevation)
    }
}

impl Elevation {
    pub fn lerp(&self, other: &Elevation, fraction: f32) -> Elevation {
        Elevation {
            elevation: self.elevation.lerp(other.elevation, fraction),
        }
    }
}

type ElevationFilter = (
    With<GameObjectId>,
    Or<(
        Changed<GameObjectId>,
        Changed<Visibility>,
        Changed<TilePosition>,
    )>,
);

pub(crate) fn apply_elevation(
    visual_elements: Res<VisualElements>,
    q_tile: Query<(&TilePosition, &Layer), ElevationFilter>,
    mut q_entities: Query<(&mut Elevation, &GameObjectId, Option<&Visibility>)>,
    map_tiles: Res<MapTiles<Entity>>,
) {
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
                let Ok((mut elevation, object_id, visibility)) = q_entities.get_mut(entity) else {
                    return tile_elevation;
                };
                let Some((group, id)) = object_id.as_group_and_id() else {
                    return tile_elevation;
                };

                let elevation_delta =
                    if visibility.cloned().unwrap_or_default() != Visibility::Hidden {
                        visual_elements
                            .get_for_group_and_id(group, id)
                            .cloned()
                            .map(|app| app.properties.elevation)
                            .unwrap_or(0) as f32
                            / SPRITE_BASE_SIZE.y as f32
                    } else {
                        0.
                    };

                elevation.elevation = match group {
                    EntityType::Object => tile_elevation,
                    EntityType::Outfit => tile_elevation,
                    EntityType::Effect => 0.,
                    EntityType::Missile => 0.,
                };

                tile_elevation + elevation_delta
            });
    }
}
