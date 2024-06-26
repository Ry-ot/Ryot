//! Systems responsible for deleting visual entities from the map using the ECS API.
//! The systems are set to run after the update systems, so that the entities are updated before
//! they are deleted. Their interaction is controlled by the state of Deletion component.
use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bevy_render::prelude::*;

#[cfg(feature = "lmdb")]
use bevy_utils::tracing::error;

#[cfg(feature = "lmdb")]
use bevy_utils::HashMap;

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
    mut q_deleted: Query<(&TilePosition, &Layer, &mut Deletion), Changed<Deletion>>,
) {
    #[cfg(feature = "lmdb")]
    {
        let Some(lmdb_env) = &lmdb_env.0 else {
            return;
        };

        let mut layer_per_pos: HashMap<Vec<u8>, Layer> = HashMap::new();

        for (tile_pos, layer, deletion) in q_deleted.iter() {
            if !deletion.state.persisted {
                layer_per_pos.insert(tile_pos.get_binary_key(), *layer);
            }
        }

        let item_repository = ItemsFromHeedLmdb::new(lmdb_env.clone());

        let tiles =
            item_repository.get_for_keys(layer_per_pos.clone().into_keys().collect::<Vec<_>>());

        if let Err(e) = tiles {
            error!("Failed to get tiles: {}", e);
            return;
        };

        let mut to_update = vec![];
        let mut to_delete = vec![];

        for tile in tiles.unwrap().iter_mut() {
            let key = tile.position.get_binary_key();

            let Some(layer) = layer_per_pos.get(&key) else {
                continue;
            };

            tile.items.remove(layer);

            // We only delete if no items are left in the tile
            // Otherwise we update the tile with the new content
            if tile.items.is_empty() {
                to_delete.push(key);
            } else {
                to_update.push(tile.clone());
            }
        }

        if let Err(e) = item_repository.save_from_tiles(to_update) {
            error!("Failed to delete tile: {}", e);
            return;
        }

        if let Err(e) = item_repository.delete_multiple(to_delete) {
            error!("Failed to delete tile: {}", e);
            return;
        }
    }

    for (.., mut deletion) in q_deleted.iter_mut() {
        if !deletion.state.persisted {
            deletion.state.persisted = true;
        }
    }
}
