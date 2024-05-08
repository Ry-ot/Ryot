use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;
use bevy_utils::HashMap;
use ryot_core::prelude::*;
use ryot_utils::prelude::*;

pub fn collect_updatable_positions<P: Into<TilePosition> + Copy + Component>(
    map_tiles: Res<MapTiles<Entity>>,
    q_updated_entities: Query<
        &P,
        Or<(
            Changed<ContentId>,
            Changed<Visibility>,
            Changed<TilePosition>,
        )>,
    >,
) -> HashMap<TilePosition, (Vec<Entity>, bool)> {
    let mut pos_map: HashMap<TilePosition, (Vec<Entity>, bool)> = HashMap::new();

    for pos in q_updated_entities.iter() {
        let tile_pos: TilePosition = (*pos).into();

        if pos_map.contains_key(&tile_pos) {
            continue;
        }

        let Some(tile) = map_tiles.get(&tile_pos) else {
            continue;
        };

        pos_map.insert(
            tile_pos,
            (tile.into_iter().map(|(_, entity)| entity).collect(), true),
        );
    }

    pos_map
}

pub fn build_new_flags_for_map<N: Navigable + Copy + Default + Component>(
    In(entities_per_pos): In<HashMap<TilePosition, (Vec<Entity>, bool)>>,
    visual_elements: Res<VisualElements>,
    q_object_and_visibility: Query<(&ContentId, Option<&Visibility>, Option<&N>)>,
) -> HashMap<TilePosition, N> {
    entities_per_pos
        .into_iter()
        .map(|(pos, (entities, append_entity_flags))| {
            (
                pos,
                entities.iter().fold(N::default(), |mut flags, entity| {
                    let Ok((object_id, visibility, entity_flags)) =
                        q_object_and_visibility.get(*entity)
                    else {
                        return flags;
                    };

                    if visibility == Some(&Visibility::Hidden) {
                        return flags;
                    }

                    if append_entity_flags {
                        flags =
                            entity_flags.map_or(flags, |e_flags| append_navigable(flags, e_flags));
                    }

                    object_id
                        .as_group_and_id()
                        .and_then(|(group, id)| visual_elements.get_for_group_and_id(group, id))
                        .map(|visual_element| visual_element.flags)
                        .filter(|&flags| !flags.is_default())
                        .map_or_else(|| flags, |a_flags| append_navigable(flags, &a_flags))
                }),
            )
        })
        .collect::<HashMap<TilePosition, N>>()
}

pub fn update_tile_flag_cache<N: Navigable>(
    In(flags_per_pos): In<HashMap<TilePosition, N>>,
    cache: ResMut<Cache<TilePosition, N>>,
) {
    let Ok(mut write_guard) = cache.write() else {
        return;
    };

    flags_per_pos.into_iter().for_each(|(pos, flags)| {
        write_guard.insert(pos, flags);
    });
}
