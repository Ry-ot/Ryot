use crate::Cursor;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use ryot::bevy_ryot::drawing::DrawingBundle;
use ryot::prelude::drawing::*;

pub fn update_brush(
    mut cursor_query: Query<(
        &mut Cursor,
        &ActionState<crate::bevy_compass::drawing::DrawingAction>,
    )>,
    brushes: Res<Brushes<DrawingBundle>>,
) {
    for (mut cursor, action_state) in cursor_query.iter_mut() {
        if action_state.just_pressed(crate::bevy_compass::drawing::DrawingAction::ChangeBrush) {
            cursor.drawing_state.brush_index = brushes.next_index(cursor.drawing_state.brush_index)
        }

        let mut size = cursor.drawing_state.brush_size;

        if action_state.just_pressed(crate::bevy_compass::drawing::DrawingAction::IncreaseBrush) {
            size += 1;
        } else if action_state
            .just_pressed(crate::bevy_compass::drawing::DrawingAction::DecreaseBrush)
        {
            size -= 1;
        }

        cursor.drawing_state.brush_size = size.clamp(0, 50);
    }
}
