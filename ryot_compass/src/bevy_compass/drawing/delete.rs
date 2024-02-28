use crate::{CommandHistory, Cursor, ToolMode};
use bevy::prelude::*;
use ryot::prelude::drawing::*;

/// System responsible for toggling the deletion mode. This system is called when the user presses the
/// [`ToggleDeletion`](crate::CompassAction::ToggleDeletion) action. When the deletion mode is active
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
    to_delete: Vec<DrawingBundle>,
    commands: &mut Commands,
    command_history: &mut ResMut<CommandHistory>,
) {
    let command = UpdateTileContent::for_new_bundle(to_delete).revert();

    commands.add(command.clone());
    command_history.reversed_commands.clear();
    command_history.performed_commands.push(command.into());
}
