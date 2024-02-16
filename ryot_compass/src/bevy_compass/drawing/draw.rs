use crate::{Cursor, DrawingAction, DrawingMode};
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::ecs::schedule::SystemConfigs;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use ryot::bevy_ryot::map::MapTiles;
use ryot::bevy_ryot::*;
use ryot::layer::Layer;
use ryot::prelude::{drawing::*, position::*};

#[cfg(feature = "lmdb")]
use ryot::bevy_ryot::lmdb::LmdbEnv;
#[cfg(feature = "lmdb")]
use ryot::lmdb::{GetKey, Item, ItemRepository, ItemsFromHeedLmdb, Tile};
#[cfg(feature = "lmdb")]
use std::collections::HashMap;
#[cfg(feature = "lmdb")]
use time_test::time_test;

pub fn draw_on_hold<C: ContentAssets>() -> SystemConfigs {
    on_hold(
        draw_to_tile::<C, Changed<TilePosition>>,
        DrawingAction::Draw,
    )
}

pub fn draw_on_click<C: ContentAssets>() -> SystemConfigs {
    on_press(draw_to_tile::<C, ()>, DrawingAction::Draw)
}

#[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
type LmdbResource<'a> = ResMut<'a, LmdbEnv>;

#[cfg(not(feature = "lmdb"))]
type LmdbResource<'a> = ();

#[allow(clippy::too_many_arguments)]
fn draw_to_tile<C: ContentAssets, F: ReadOnlyWorldQuery>(
    mut commands: Commands,
    mut tiles: ResMut<MapTiles>,
    lmdb_env: LmdbResource,
    mut command_history: ResMut<CommandHistory>,
    content_assets: Res<C>,
    brushes: Res<Brushes<DrawingBundle>>,
    current_appearance_query: Query<(&mut AppearanceDescriptor, &Visibility), Without<Cursor>>,
    cursor_query: Query<(&AppearanceDescriptor, &TilePosition, &Cursor), F>,
) {
    if content_assets.sprite_sheet_data_set().is_none() {
        warn!("Trying to draw a sprite without any loaded content");
        return;
    };

    for (AppearanceDescriptor { group, id, .. }, tile_pos, cursor) in &cursor_query {
        if !cursor.drawing_state.enabled {
            continue;
        }

        let Some(prepared_appearance) = content_assets
            .prepared_appearances()
            .get_for_group(*group, *id)
        else {
            return;
        };

        let layer = prepared_appearance.layer;
        let appearance = AppearanceDescriptor::new(*group, *id, default());

        let to_draw = brushes(
            cursor.drawing_state.brush_index,
            cursor.drawing_state.mode.into(),
            DrawingBundle::new(layer, *tile_pos, appearance),
        );

        let mut queued = 0;

        for new_bundle in &to_draw {
            let Some(command) = create_and_send_update_command(
                layer,
                *new_bundle,
                &mut commands,
                &mut tiles,
                &current_appearance_query,
            ) else {
                continue;
            };

            command_history
                .performed_commands
                .push(TileCommandRecord::new(layer, new_bundle.tile_pos, Box::new(command)).into());

            queued += 1;
        }

        command_history
            .performed_commands
            .push(CommandBatchSize(queued).into());

        command_history.reversed_commands.clear();

        #[cfg(feature = "lmdb")]
        {
            time_test!("Writing");
            let item_repository = ItemsFromHeedLmdb::new(lmdb_env.clone());
            let mut new_tiles: HashMap<TilePosition, Tile> = HashMap::new();

            let binary_keys = to_draw
                .iter()
                .map(|bundle| bundle.tile_pos.get_binary_key())
                .collect();

            let Ok(tiles) = item_repository.get_for_keys(binary_keys) else {
                continue;
            };

            for tile in tiles {
                new_tiles.insert(tile.position, tile);
            }

            for bundle in &to_draw {
                let tile = new_tiles
                    .entry(bundle.tile_pos)
                    .or_insert(Tile::from_pos(bundle.tile_pos));

                tile.set_item(
                    Item {
                        id: bundle.appearance.id as u16,
                        attributes: vec![],
                    },
                    bundle.layer,
                );
            }

            if let Err(e) = item_repository.save_from_tiles(new_tiles.into_values().collect()) {
                error!("Failed to save tile: {}", e);
            }
        };
    }
}

pub fn create_and_send_update_command(
    layer: Layer,
    new_bundle: DrawingBundle,
    commands: &mut Commands,
    tiles: &mut ResMut<MapTiles>,
    current_appearance_query: &Query<(&mut AppearanceDescriptor, &Visibility), Without<Cursor>>,
) -> Option<UpdateTileContent> {
    let entity = tiles
        .entry(new_bundle.tile_pos)
        .or_default()
        .get(&layer)
        .map_or_else(|| commands.spawn_empty().id(), |&e| e);

    let old_bundle = match current_appearance_query.get(entity) {
        Ok((appearance, visibility)) => Some(
            DrawingBundle::new(layer, new_bundle.tile_pos, *appearance)
                .with_visibility(*visibility),
        ),
        Err(_) => None,
    };

    if old_bundle == Some(new_bundle) {
        return None;
    }

    let command = UpdateTileContent(Some(new_bundle), old_bundle);
    commands.add(command.with_entity(entity));

    Some(command)
}

pub fn update_drawing_mode(mut cursor_query: Query<(&TilePosition, &mut Cursor)>) {
    for (cursor_pos, mut cursor) in &mut cursor_query {
        cursor.drawing_state.mode = match cursor.drawing_state.mode {
            DrawingMode::TwoClicks(Some(_)) => DrawingMode::TwoClicks(None),
            DrawingMode::TwoClicks(None) => DrawingMode::TwoClicks(Some(*cursor_pos)),
            mode => mode,
        };
    }
}

pub fn change_drawing_mode(mut cursor_query: Query<&mut Cursor>) {
    for mut cursor in &mut cursor_query {
        cursor.drawing_state.mode = match cursor.drawing_state.mode {
            DrawingMode::TwoClicks(_) => DrawingMode::Click(3),
            DrawingMode::Click(_) => DrawingMode::TwoClicks(None),
        };
    }
}

pub fn clear_selection(mut cursor_query: Query<&mut Cursor>) {
    for mut cursor in &mut cursor_query {
        cursor.drawing_state.mode = match cursor.drawing_state.mode {
            DrawingMode::TwoClicks(Some(_)) => DrawingMode::TwoClicks(None),
            mode => mode,
        };
    }
}
