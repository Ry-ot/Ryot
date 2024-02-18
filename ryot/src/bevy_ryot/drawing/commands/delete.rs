use crate::bevy_ryot::drawing::*;
use crate::bevy_ryot::map::MapTiles;
use crate::position::TilePosition;
use bevy::ecs::system::EntityCommand;
use itertools::Itertools;

#[cfg(feature = "lmdb")]
use crate::bevy_ryot::lmdb::LmdbEnv;

#[cfg(feature = "lmdb")]
use crate::lmdb::{GetKey, ItemRepository, ItemsFromHeedLmdb};
/*
Deletion:
Send a command that adds Deletion(requested) component to the entity.
Reverting Deletion means to set Deletion(removed).
A system that read all entities with Deleted and:
 - if requested hide them, setting Deletion(done).
 - if removed show them and remove the component.

Creation/Update:
Send a command that adds a Insertion(created) or Insertion(updated) component to the entity.
Reverting a means to set Insertion(cancelled).
A system that read all entities with Insertion and:
 - if created it adds the bundles to the entity and adds it to the e
*/

#[derive(Eq, PartialEq, Component, Default, Clone, Reflect)]
pub struct Deletion {
    #[reflect(ignore)]
    pub command: DeleteTileContent,
    pub state: CommandState,
}

impl Deletion {
    pub fn for_command(command: DeleteTileContent) -> Self {
        Self {
            command,
            state: CommandState::Requested,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub struct DeleteTileContent(pub Vec<DrawingBundle>);

impl From<DeleteTileContent> for CommandType {
    fn from(command: DeleteTileContent) -> Self {
        Self::TileCommand(TileCommandRecord::new(
            command.0.first().unwrap().layer,
            command.0.first().unwrap().tile_pos,
            Box::new(command),
        ))
    }
}

impl EntityCommand for DeleteTileContent {
    fn apply(self, id: Entity, world: &mut World) {
        world.entity_mut(id).insert(Deletion::for_command(self));
    }
}

impl ReversibleCommand for DeleteTileContent {
    fn undo(&self, commands: &mut Commands, entity: Option<Entity>) {
        let Some(entity) = entity else {
            return;
        };

        let Some(bundle) = self.0.first() else {
            return;
        };

        commands.add(CreateTileContent(*bundle).with_entity(entity));
    }

    fn redo(&self, commands: &mut Commands, entity: Option<Entity>) {
        if let Some(entity) = entity {
            commands.add(self.clone().with_entity(entity));
        }
    }
}

pub fn apply_deletion(
    mut q_deleted: Query<
        (&mut Visibility, &mut Deletion),
        Or<(Changed<Deletion>, Added<Deletion>)>,
    >,
) {
    for (mut visibility, mut deletion) in q_deleted.iter_mut() {
        if deletion.state == CommandState::Requested {
            *visibility = Visibility::Hidden;
            deletion.state = CommandState::Applied;
        }
    }
}

pub fn persist_deletion(
    #[cfg(feature = "lmdb")] lmdb_env: ResMut<LmdbEnv>,
    mut q_deleted: Query<(&TilePosition, &mut Deletion), Changed<Deletion>>,
) {
    #[cfg(feature = "lmdb")]
    {
        let mut keys = vec![];

        for (tile_pos, deletion) in q_deleted.iter() {
            if deletion.state == CommandState::Applied {
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
        if deletion.state == CommandState::Applied {
            deletion.state = CommandState::Persisted;
        }
    }
}

pub fn get_top_most_visible(
    tile_pos: TilePosition,
    map_tiles: &Res<MapTiles>,
    q_visibility: &Query<(&Visibility, &AppearanceDescriptor), With<TileComponent>>,
) -> Option<(Entity, DrawingBundle)> {
    let Some(tile_content) = map_tiles.get(&tile_pos) else {
        return None;
    };

    let tile_content = tile_content.clone();

    let top_most_keys = tile_content
        .keys()
        .sorted_by_key(|layer| std::cmp::Reverse(*layer))
        .copied()
        .collect::<Vec<_>>();

    for layer in top_most_keys {
        let entity = tile_content.get(&layer).unwrap();

        if let Ok((visibility, appearance)) = q_visibility.get(*entity) {
            if visibility == Visibility::Hidden {
                continue;
            }

            return Some((*entity, DrawingBundle::new(layer, tile_pos, *appearance)));
        }
    }

    None
}

pub fn get_bottom_most_deleted(world: &mut World, tile_pos: TilePosition) -> Option<Entity> {
    let map_tiles = world.resource_mut::<MapTiles>();

    let Some(tile_content) = map_tiles.get(&tile_pos) else {
        return None;
    };

    let tile_content = tile_content.clone();

    let bottom_most_keys = tile_content
        .keys()
        .sorted_by_key(|layer| *layer)
        .copied()
        .collect::<Vec<_>>();

    for layer in bottom_most_keys {
        let entity = tile_content.get(&layer).unwrap();

        if world.get::<Deletion>(*entity).is_some() {
            return Some(*entity);
        }
    }

    None
}
