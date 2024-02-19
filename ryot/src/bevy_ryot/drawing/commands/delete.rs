use crate::bevy_ryot::drawing::*;
use crate::bevy_ryot::map::MapTiles;
use crate::position::TilePosition;
use itertools::Itertools;

#[cfg(feature = "lmdb")]
use crate::bevy_ryot::lmdb::LmdbEnv;

#[cfg(feature = "lmdb")]
use crate::lmdb::{GetKey, ItemRepository, ItemsFromHeedLmdb};

#[derive(Eq, PartialEq, Component, Default, Clone, Reflect)]
pub struct Deletion {
    pub state: CommandState,
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
    let tile_content = map_tiles.get(&tile_pos)?.clone();

    let top_most_keys = tile_content
        .keys()
        .sorted_by_key(|layer| std::cmp::Reverse(*layer))
        .copied()
        .collect::<Vec<_>>();

    for layer in top_most_keys {
        let entity = tile_content.get(&layer)?;

        if let Ok((visibility, appearance)) = q_visibility.get(*entity) {
            if visibility == Visibility::Hidden {
                continue;
            }

            return Some((*entity, DrawingBundle::new(layer, tile_pos, *appearance)));
        }
    }

    None
}
