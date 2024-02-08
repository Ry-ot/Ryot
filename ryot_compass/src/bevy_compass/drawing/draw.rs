use crate::{Brush, Cursor, DrawingAction, SquareBrush};
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use ryot::bevy_ryot::*;
use ryot::prelude::{drawing::*, position::*};

pub(super) fn draw_to_tile<C: ContentAssets>(
    mut commands: Commands,
    mut tiles: ResMut<MapTiles>,
    mut command_history: ResMut<CommandHistory>,
    content_assets: Res<C>,
    current_appearance_query: Query<(&mut AppearanceDescriptor, &Visibility), Without<Cursor>>,
    cursor_query: Query<(
        &ActionState<DrawingAction>,
        &AppearanceDescriptor,
        &TilePosition,
        &Cursor,
        Changed<TilePosition>,
    )>,
) {
    if content_assets.sprite_sheet_data_set().is_none() {
        warn!("Trying to draw a sprite without any loaded content");
        return;
    };

    for (
        action_state,
        AppearanceDescriptor { group, id, .. },
        tile_pos,
        cursor,
        position_changed,
    ) in &cursor_query
    {
        if !cursor.drawing_state.enabled {
            continue;
        }

        let Some(prepared_appearance) = content_assets
            .prepared_appearances()
            .get_for_group(*group, *id)
        else {
            return;
        };

        if !check_action(DrawingAction::Draw, position_changed, action_state) {
            return;
        }

        let layer = prepared_appearance.layer;
        let appearance = AppearanceDescriptor::new(*group, *id, default());

        let bundles = SquareBrush::new(5).apply(DrawingBundle::new(layer, *tile_pos, appearance));
        let mut queued = 0;

        for new_bundle in bundles {
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
    }
}
