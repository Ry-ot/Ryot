use crate::{Cursor, DrawingAction};
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::ecs::schedule::SystemConfigs;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use ryot::bevy_ryot::map::MapTiles;
use ryot::bevy_ryot::*;
use ryot::prelude::{drawing::*, position::*};

pub fn draw_on_hold<C: ContentAssets>() -> SystemConfigs {
    on_hold(
        draw_to_tile::<C, Changed<TilePosition>>,
        DrawingAction::Draw,
    )
}

pub fn draw_on_click<C: ContentAssets>() -> SystemConfigs {
    on_press(draw_to_tile::<C, ()>, DrawingAction::Draw)
}

fn draw_to_tile<C: ContentAssets, F: ReadOnlyWorldQuery>(
    mut commands: Commands,
    mut tiles: ResMut<MapTiles>,
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
            cursor.drawing_state.brush_size,
            DrawingBundle::new(layer, *tile_pos, appearance),
        );

        let mut queued = 0;

        for new_bundle in to_draw {
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
                continue;
            }

            let command = UpdateTileContent(Some(new_bundle), old_bundle);
            commands.add(command.with_entity(entity));
            command_history
                .performed_commands
                .push(TileCommandRecord::new(layer, new_bundle.tile_pos, Box::new(command)).into());

            queued += 1;
        }

        command_history
            .performed_commands
            .push(CommandBatchSize(queued).into());

        command_history.reversed_commands.clear();
    }
}
