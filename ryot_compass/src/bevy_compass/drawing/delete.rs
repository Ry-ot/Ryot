use crate::{Cursor, DrawingAction};
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::ecs::schedule::SystemConfigs;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use ryot::bevy_ryot::map::MapTiles;
use ryot::bevy_ryot::*;
use ryot::prelude::{drawing::*, position::*};

pub fn erase_on_hold() -> SystemConfigs {
    on_hold(
        delete_tile_content::<Changed<TilePosition>>,
        DrawingAction::Erase,
    )
}

pub fn erase_on_click() -> SystemConfigs {
    on_press(delete_tile_content::<()>, DrawingAction::Erase)
}

/// A function that listens to the right mouse button and deletes the content of the tile under the cursor.
/// It always delete the topmost content of the tile, following the Z-ordering.

#[allow(clippy::too_many_arguments)]
fn delete_tile_content<F: ReadOnlyWorldQuery>(
    mut commands: Commands,
    mut command_history: ResMut<CommandHistory>,
    tiles: Res<MapTiles>,
    brushes: Res<Brushes<DrawingBundle>>,
    cursor_query: Query<(&Cursor, &TilePosition), F>,
    q_visibility: Query<(&Visibility, &AppearanceDescriptor), With<TileComponent>>,
) {
    for (cursor, tile_pos) in &cursor_query {
        let positions: Vec<TilePosition> = brushes(
            cursor.drawing_state.brush_index,
            cursor.drawing_state.mode.into(),
            DrawingBundle::from_tile_position(*tile_pos),
        )
        .into_iter()
        .map(|bundle| bundle.tile_pos)
        .collect();

        let top_most_content = positions
            .iter()
            .filter_map(|pos| get_top_most_visible(*pos, &tiles, &q_visibility))
            .collect::<Vec<_>>();

        top_most_content.iter().for_each(|(entity, bundle)| {
            let command = DeleteTileContent(vec![*bundle]);
            commands.add(command.clone().with_entity(*entity));
            command_history.performed_commands.push(command.into());
        });

        command_history
            .performed_commands
            .push(CommandBatchSize(top_most_content.len()).into());

        command_history.reversed_commands.clear();
    }
}
