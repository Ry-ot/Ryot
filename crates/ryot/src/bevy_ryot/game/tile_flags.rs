//! This module deals with the definition and management of `TileFlags`, which represent the state of tiles within the game world.
//! These flags are crucial for determining visibility, walkability, and whether a tile blocks sight, among other properties.
use bevy::prelude::*;
use ryot_core::prelude::*;

use crate::bevy_ryot::*;

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
            .add_systems(PostUpdate, update_tile_flag_cache::<C>);
    }
}

/// Represents flags associated with a tile, including its visibility to players, walkability,
/// and whether it obstructs the line of sight. These properties are essential for gameplay mechanics.
#[derive(Debug, Clone, Component, Copy, Eq, PartialEq, Reflect)]
pub struct TileFlags {
    walkable: bool,
    blocks_sight: bool,
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
    pub fn new(walkable: bool, blocks_sight: bool) -> Self {
        TileFlags {
            walkable,
            blocks_sight,
        }
    }

    pub fn with_walkable(self, walkable: bool) -> Self {
        TileFlags { walkable, ..self }
    }

    pub fn with_blocks_sight(self, blocks_sight: bool) -> Self {
        TileFlags {
            blocks_sight,
            ..self
        }
    }

    /// Updates the flags based on the appearance attributes of the tile.
    /// This allows dynamic modification of tile properties based on in-game events or conditions.
    pub fn for_appearance_flags(self, flags: Flags) -> Self {
        self.append(TileFlags {
            walkable: !is_true(flags.is_not_walkable),
            blocks_sight: is_true(flags.blocks_sight),
        })
    }

    pub fn append(self, flags: TileFlags) -> Self {
        TileFlags {
            walkable: self.walkable && flags.walkable,
            blocks_sight: self.blocks_sight || flags.blocks_sight,
        }
    }
}

impl Flag for TileFlags {
    fn is_walkable(&self) -> bool {
        self.walkable
    }

    fn blocks_sight(&self) -> bool {
        self.blocks_sight
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
        (Option<&PreviousPosition>, &TilePosition),
        Or<(
            Changed<GameObjectId>,
            Changed<Visibility>,
            Changed<TilePosition>,
        )>,
    >,
    q_object_and_visibility: Query<(&GameObjectId, Option<&Visibility>, Option<&TileFlags>)>,
) {
    let appearances = appearance_assets.prepared_appearances();

    for (previous_pos, new_pos) in q_updated_entities.iter() {
        let previous_pos = match previous_pos {
            Some(previous_pos) => *previous_pos,
            None => PreviousPosition(*new_pos),
        };

        let positions = if previous_pos.0 == *new_pos {
            vec![*new_pos]
        } else {
            vec![previous_pos.0, *new_pos]
        };

        for pos in &positions {
            let Some(tile) = map_tiles.get(pos) else {
                continue;
            };

            cache.insert(
                *pos,
                tile.into_iter()
                    .fold(TileFlags::default(), |mut flags, (_, entity)| {
                        let Ok((object_id, visibility, entity_flags)) =
                            q_object_and_visibility.get(entity)
                        else {
                            return flags;
                        };

                        if visibility == Some(&Visibility::Hidden) {
                            return flags;
                        }

                        if let Some(entity_flags) = entity_flags
                            && pos == new_pos
                        {
                            flags = flags.append(*entity_flags);
                        }

                        let appearance_flags = || -> Option<Flags> {
                            let (group, id) = object_id.as_group_and_id()?;
                            let appearance = appearances.get_for_group(group, id).cloned()?;
                            appearance.flags
                        };

                        if let Some(appearance_flags) = appearance_flags() {
                            flags = flags.for_appearance_flags(appearance_flags);
                        };

                        flags
                    }),
            );
        }
    }
}
