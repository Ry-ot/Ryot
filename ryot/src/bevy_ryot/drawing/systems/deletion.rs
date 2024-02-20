//! Systems responsible for deleting visual entities from the map using the ECS API.
//! The systems are set to run after the update systems, so that the entities are updated before
//! they are deleted. Their interaction is controlled by the state of Deletion component.

use crate::bevy_ryot::drawing::*;
use crate::bevy_ryot::map::MapTiles;
use crate::position::TilePosition;
use itertools::Itertools;

#[cfg(feature = "lmdb")]
use crate::bevy_ryot::lmdb::LmdbEnv;

#[cfg(feature = "lmdb")]
use crate::lmdb::{GetKey, ItemRepository, ItemsFromHeedLmdb};

/// A component that flags the entity to be deleted from the map and controls the state
/// of the deletion. The state is used to control the deletion flows and avoid deleting
/// the same entity multiple times.
///
/// Runs during [`Apply`](DrawingSystems::Apply) and before [`Persist`](DrawingSystems::Persist).
#[derive(Eq, PartialEq, Component, Default, Clone, Reflect)]
pub struct Deletion {
    pub state: CommandState,
}

/// A system that applies the deletion to the entities that are marked for deletion.
/// Apply means to performed the needed actions to delete the entity from the map.
pub fn apply_deletion(
    mut q_deleted: Query<
        (&mut Visibility, &mut Deletion),
        Or<(Changed<Deletion>, Added<Deletion>)>,
    >,
) {
    for (mut visibility, mut deletion) in q_deleted.iter_mut() {
        if !deletion.state.applied {
            *visibility = Visibility::Hidden;
            deletion.state.applied = true;
        }
    }
}

/// A system that persists the deletion of the entities that are marked for deletion.
/// Persist means to save the changes to the persistence layer, like a database or similar.
/// This implementation uses the LMDB, a key-value storage disk-based database, as the persistence
/// layer. The entities are deleted from the LMDB using the TilePosition as the key.
///
/// Runs during [`Persist`](DrawingSystems::Persist) and after [`Apply`](DrawingSystems::Apply).
pub fn persist_deletion(
    #[cfg(feature = "lmdb")] lmdb_env: ResMut<LmdbEnv>,
    mut q_deleted: Query<(&TilePosition, &mut Deletion), Changed<Deletion>>,
) {
    #[cfg(feature = "lmdb")]
    {
        let mut keys = vec![];

        for (tile_pos, deletion) in q_deleted.iter() {
            if !deletion.state.persisted {
                keys.push(tile_pos.get_binary_key());
            }
        }

        let item_repository = ItemsFromHeedLmdb::new(lmdb_env.clone());

        if let Err(e) = item_repository.delete_multiple(keys) {
            error!("Failed to delete tile: {}", e);
            return;
        }
    }

    for (_, mut deletion) in q_deleted.iter_mut() {
        if !deletion.state.persisted {
            deletion.state.persisted = true;
        }
    }
}

/// Auxiliary function to get the top most visible entity and its DrawingBundle from a tile position.
/// Deletion means to always remove the entity that is on the top most layer and is visible.
/// This can be use by the application to determine which entity should be flagged for deletion when a
/// deletion action is performed in a Tile.
pub fn get_top_most_visible(
    tile_pos: TilePosition,
    map_tiles: &ResMut<MapTiles>,
    q_current_appearance: &Query<(&Visibility, &AppearanceDescriptor), With<TileComponent>>,
) -> Option<(Entity, DrawingBundle)> {
    let tile_content = map_tiles.get(&tile_pos)?.clone();

    let top_most_keys = tile_content
        .keys()
        .sorted_by_key(|layer| std::cmp::Reverse(*layer))
        .copied()
        .collect::<Vec<_>>();

    for layer in top_most_keys {
        let entity = tile_content.get(&layer)?;

        if let Ok((visibility, appearance)) = q_current_appearance.get(*entity) {
            if visibility == Visibility::Hidden {
                continue;
            }

            return Some((*entity, DrawingBundle::new(layer, tile_pos, *appearance)));
        }
    }

    None
}
