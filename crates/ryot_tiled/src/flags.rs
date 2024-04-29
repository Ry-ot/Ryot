use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;
use ryot_core::prelude::*;
use ryot_utils::prelude::*;

pub fn update_tile_flag_cache<N: Navigable + Copy + Default + Component>(
    visual_elements: Res<VisualElements>,
    map_tiles: Res<MapTiles<Entity>>,
    cache: ResMut<Cache<TilePosition, N>>,
    q_updated_entities: Query<
        (Option<&PreviousPosition>, &TilePosition),
        Or<(
            Changed<ContentId>,
            Changed<Visibility>,
            Changed<TilePosition>,
        )>,
    >,
    q_object_and_visibility: Query<(&ContentId, Option<&Visibility>, Option<&N>)>,
) {
    let Ok(mut write_guard) = cache.write() else {
        return;
    };

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

            write_guard.insert(
                *pos,
                tile.into_iter()
                    .fold(N::default(), |mut flags, (_, entity)| {
                        let Ok((object_id, visibility, entity_flags)) =
                            q_object_and_visibility.get(entity)
                        else {
                            return flags;
                        };

                        if visibility == Some(&Visibility::Hidden) {
                            return flags;
                        }

                        if pos == new_pos {
                            flags = entity_flags
                                .map_or_else(|| flags, |entity_flags| flags.append(entity_flags));
                        }

                        flags = object_id
                            .as_group_and_id()
                            .and_then(|(group, id)| visual_elements.get_for_group_and_id(group, id))
                            .map(|visual_element| visual_element.flags)
                            .filter(|&flags| !flags.is_default())
                            .map_or_else(|| flags, |a_flags| flags.append(&a_flags));

                        flags
                    }),
            );
        }
    }
}
