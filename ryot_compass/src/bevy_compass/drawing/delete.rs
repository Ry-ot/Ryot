use crate::{Cursor, DrawingAction};
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::ecs::schedule::SystemConfigs;
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
fn delete_tile_content<F: ReadOnlyWorldQuery>(
    mut commands: Commands,
    mut command_history: ResMut<CommandHistory>,
    tiles: ResMut<MapTiles>,
    brushes: Res<Brushes<DrawingBundle>>,
    cursor_query: Query<(&Cursor, &TilePosition), F>,
    q_current_appearance: Query<(&Visibility, &AppearanceDescriptor), With<TileComponent>>,
) {
    for (cursor, tile_pos) in &cursor_query {
        let top_most_content = brushes(
            cursor.drawing_state.brush_index,
            cursor.drawing_state.input_type.into(),
            DrawingBundle::from_tile_position(*tile_pos),
        )
        .iter()
        .filter_map(|bundle| get_top_most_visible(bundle.tile_pos, &tiles, &q_current_appearance))
        .map(|(_, bundle)| bundle)
        .collect::<Vec<_>>();

        let command = UpdateTileContent::for_new_bundle(top_most_content).revert();
        commands.add(command.clone());

        command_history.reversed_commands.clear();
        command_history.performed_commands.push(command.into());
    }
}
