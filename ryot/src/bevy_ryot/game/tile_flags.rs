//! This module deals with the definition and management of `TileFlags`, which represent the state of tiles within the game world.
//! These flags are crucial for determining visibility, walkability, and whether a tile blocks sight, among other properties.
use bevy::prelude::*;

use crate::appearances::*;
use crate::bevy_ryot::map::MapTiles;
use crate::bevy_ryot::*;
use crate::position::TilePosition;

/// `TileFlagPlugin` provides the necessary system and resource setup for managing `TileFlags`
/// within the game world. It ensures that the flag cache is up-to-date and reflects the latest
/// flag state of the whole tile, per position. This avoids the need to iterate over each entity
/// within a tile to check its properties.
pub struct TileFlagPlugin<C: AppearanceAssets>(PhantomData<C>);

impl<C: AppearanceAssets> Default for TileFlagPlugin<C> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C: AppearanceAssets> Plugin for TileFlagPlugin<C> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cache<TilePosition, TileFlags>>()
            .add_systems(
                Update,
                update_tile_flag_cache::<C>.in_set(CacheSystems::UpdateCache),
            );
    }
}

/// Represents flags associated with a tile, including its visibility to players, walkability,
/// and whether it obstructs the line of sight. These properties are essential for gameplay mechanics.
#[derive(Debug, Clone, Component, Copy, Eq, PartialEq, Reflect)]
pub struct TileFlags {
    pub walkable: bool,
    pub blocks_sight: bool,
}

impl Default for TileFlags {
    fn default() -> Self {
        TileFlags {
            walkable: true,
            blocks_sight: false,
        }
    }
}

impl TileFlags {
    /// Updates the flags based on the appearance attributes of the tile.
    /// This allows dynamic modification of tile properties based on in-game events or conditions.
    pub fn for_appearance_flags(self, flags: Flags) -> Self {
        TileFlags {
            walkable: self.walkable && !is_true(flags.is_not_walkable),
            blocks_sight: self.blocks_sight || is_true(flags.blocks_sight),
        }
    }
}

/// Synchronizes the `TileFlags` cache with current game state changes related to visibility and object attributes.
///
/// This system plays a critical role in gameplay mechanics by dynamically updating tile properties based on
/// visibility changes and appearance attributes defined in game objects. It directly affects how entities interact
/// with the game world, particularly in terms of navigation and line-of-sight calculations.
///
/// The function leverages a cache to store `TileFlags` for each tile position, significantly optimizing
/// performance. By avoiding repetitive access to each entity within a tile to check its properties, the game
/// can quickly and efficiently update the state of the game world, ensuring accurate and up-to-date flag settings.
///
/// By maintaining an up-to-date cache of `TileFlags`, this system facilitates efficient game world interactions
/// and mechanics, enhancing the overall gameplay experience.
///
/// Run as part of [`CacheSystems::UpdateCache`].
pub fn update_tile_flag_cache<C: AppearanceAssets>(
    appearance_assets: Res<C>,
    map_tiles: Res<MapTiles<Entity>>,
    mut cache: ResMut<Cache<TilePosition, TileFlags>>,
    q_updated_entities: Query<
        &TilePosition,
        Or<(
            Changed<GameObjectId>,
            Changed<Visibility>,
            Changed<TilePosition>,
        )>,
    >,
    q_object_and_visibility: Query<(&GameObjectId, Option<&Visibility>)>,
) {
    let appearances = appearance_assets.prepared_appearances();

    for pos in q_updated_entities.iter() {
        let Some(tile) = map_tiles.get(pos) else {
            continue;
        };

        cache.insert(
            *pos,
            tile.into_iter()
                .fold(TileFlags::default(), |flags, (_, entity)| {
                    let Ok((object_id, visibility)) = q_object_and_visibility.get(entity) else {
                        return flags;
                    };

                    if visibility == Some(&Visibility::Hidden) {
                        return flags;
                    }

                    let appearance_flags = || -> Option<Flags> {
                        let (group, id) = object_id.as_group_and_id()?;
                        let appearance = appearances.get_for_group(group, id).cloned()?;
                        appearance.flags
                    };

                    let Some(appearance_flags) = appearance_flags() else {
                        return flags;
                    };

                    flags.for_appearance_flags(appearance_flags)
                }),
        );
    }
}
