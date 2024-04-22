use crate::bevy_ryot::drawing::{CommandState, Deletion, DrawingInfo, TileComponent};
use ryot_tiled::prelude::*;

use bevy::prelude::*;

#[cfg(feature = "lmdb")]
use crate::bevy_ryot::lmdb::LmdbEnv;
#[cfg(feature = "lmdb")]
use crate::bevy_ryot::GameObjectId;
#[cfg(feature = "lmdb")]
use bevy::log::error;
#[cfg(feature = "lmdb")]
use std::collections::HashMap;

/// A component that flags the entity to be updated on the map and controls the state
/// of the update. The state is used to control the update flows and avoid updating
/// the same entity multiple times. On top of the state, it also holds the new and old
/// DrawingInfo, which are used to update the entity and allows things like undo/redo.
#[derive(Eq, PartialEq, Component, Default, Copy, Clone)]
pub struct UpdateComponent {
    pub new: DrawingInfo,
    pub old: DrawingInfo,
    pub state: CommandState,
}

impl UpdateComponent {
    pub fn new(new: DrawingInfo, old: DrawingInfo) -> Self {
        Self {
            new,
            old,
            state: CommandState::default(),
        }
    }
}

/// An auxiliary function that serves as trigger to update the entities that are drawn on the map.
/// It uses World to be compatible with Commands within the Bevy ecosystem.
pub fn update(world: &mut World, new: DrawingInfo, old: DrawingInfo, state: CommandState) {
    let id = get_or_create_entity_for_info(world, &new);

    // We need to update MapTiles here already, otherwise it can lead to a race condition
    // where another entity is created before the apply_update system runs.
    world
        .resource_mut::<MapTiles<Entity>>()
        .entry(new.0)
        .or_default()
        .push_for_layer(new.1, id);

    world
        .entity_mut(id)
        .insert(UpdateComponent { new, old, state });
}

/// An auxiliary function that fetches or create from the map of tiles the entity related to the
/// DrawingInfo. It is used to avoid creating multiple entities for the same tile position and layer.
pub fn get_or_create_entity_for_info(world: &mut World, info: &DrawingInfo) -> Entity {
    let (pos, layer, ..) = info;

    let entity = world
        .resource_mut::<MapTiles<Entity>>()
        .entry(*pos)
        .or_default()
        .peek_for_layer(*layer);

    match entity {
        Some(entity) => entity,
        None => world.spawn_empty().id(),
    }
}

/// A system that applies the update to the entities that are marked for update.
/// Apply means to performed the needed actions to update the entity on the map.
///
/// Runs during [`Apply`](DrawingSystems::Apply) and before [`Persist`](DrawingSystems::Persist).
pub fn apply_update(
    mut commands: Commands,
    mut q_inserted: Query<
        (Entity, &mut UpdateComponent),
        Or<(Changed<UpdateComponent>, Added<UpdateComponent>)>,
    >,
) {
    for (entity, mut update) in q_inserted.iter_mut() {
        if update.state.applied {
            continue;
        }

        let (pos, layer, visibility, id_and_group) = update.new;

        // If no id and group are provided, update is ended and the deletion is triggered.
        let Some(id_and_group) = id_and_group else {
            commands
                .entity(entity)
                .insert(Deletion::default())
                .remove::<UpdateComponent>();

            continue;
        };

        commands
            .entity(entity)
            .insert((pos, layer, id_and_group, visibility, TileComponent))
            .remove::<Deletion>();

        update.state.applied = true;
    }
}

/// A system that persists the update of the entities that are marked for update.
/// Persist means to save the changes to the persistence layer, like a database or similar.
/// This implementation uses the LMDB, a key-value storage disk-based database, as the persistence
/// layer. The entities are updated in the LMDB using the TilePosition as the key.
///
/// The layers are use to built-up the Tile information that is stored in the position key.
/// The key is [u8] representation of the TilePosition.
///
/// Runs during [`Persist`](DrawingSystems::Persist) and after [`Apply`](DrawingSystems::Apply).
pub fn persist_update(
    #[cfg(feature = "lmdb")] lmdb_env: Res<LmdbEnv>,
    mut q_inserted: Query<
        &mut UpdateComponent,
        Or<(Changed<UpdateComponent>, Added<UpdateComponent>)>,
    >,
) {
    #[cfg(feature = "lmdb")]
    {
        let Some(lmdb_env) = &lmdb_env.0 else {
            return;
        };

        let mut keys = vec![];
        let mut to_draw = vec![];

        for update in q_inserted.iter_mut() {
            let (tile_pos, layer, _, id_and_group) = update.new;

            if update.state.persisted {
                continue;
            }

            let Some(id_and_group) = id_and_group else {
                continue;
            };

            keys.push(tile_pos.get_binary_key());
            to_draw.push((tile_pos, layer, id_and_group));
        }

        let item_repository = ItemsFromHeedLmdb::new(lmdb_env.clone());
        let mut new_tiles: HashMap<TilePosition, Tile> = HashMap::new();

        let tiles = item_repository.get_for_keys(keys);

        if let Err(e) = tiles {
            error!("Failed to get tiles: {}", e);
            return;
        };

        for tile in tiles.unwrap() {
            new_tiles.insert(tile.position, tile);
        }

        for (tile_pos, layer, (object_id, _frame_group)) in &to_draw {
            let tile = new_tiles
                .entry(*tile_pos)
                .or_insert(Tile::from_pos(*tile_pos));

            let id = match object_id {
                GameObjectId::Object(id) => *id as u16,
                _ => continue,
            };

            tile.set_item(
                Item {
                    id,
                    attributes: vec![],
                },
                *layer,
            );
        }

        if let Err(e) = item_repository.save_from_tiles(new_tiles.into_values().collect()) {
            error!("Failed to save tile: {}", e);
        }
    }

    for mut update in q_inserted.iter_mut() {
        if update.state.persisted {
            continue;
        }

        update.state.persisted = true;
    }
}
