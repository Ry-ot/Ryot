use crate::{CommandHistory, Cursor, ToolMode};
use bevy::prelude::*;
use ryot::bevy_ryot::map::MapTiles;
use ryot::bevy_ryot::*;
use ryot::prelude::drawing::*;

/// System responsible for toggling the deletion mode. This system is called when the user presses the
/// [`ToggleDeletion`](crate::DrawingAction::ToggleDeletion) action. When the deletion mode is active
/// the inputs are used to delete the top most elements in the positions where the cursor is.
pub fn toggle_deletion(mut q_cursor: Query<&mut Cursor>) {
    for mut cursor in q_cursor.iter_mut() {
        cursor.drawing_state.tool_mode = if cursor.drawing_state.tool_mode == ToolMode::Erase {
            ToolMode::Draw
        } else {
            ToolMode::Erase
        }
    }
}

/// Auxiliary function to delete the top most elements in the positions where the cursor is.
/// This function is called when the deletion mode is active and the user clicks on the map.
pub fn delete_top_most_elements_in_positions(
    to_delete: &[DrawingBundle],
    commands: &mut Commands,
    command_history: &mut ResMut<CommandHistory>,
    tiles: &ResMut<MapTiles>,
    q_current_appearance: &Query<(&Visibility, &AppearanceDescriptor), With<TileComponent>>,
) {
    let top_most_content = to_delete
        .iter()
        .filter_map(|bundle| get_top_most_visible(bundle.tile_pos, tiles, q_current_appearance))
        .map(|(_, bundle)| bundle)
        .collect::<Vec<_>>();

    let command = UpdateTileContent::for_new_bundle(top_most_content).revert();
    commands.add(command.clone());

    command_history.reversed_commands.clear();
    command_history.performed_commands.push(command.into());
}
