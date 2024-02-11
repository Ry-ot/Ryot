use crate::{Cursor, DrawingAction};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use ryot::bevy_ryot::drawing::DrawingBundle;
use ryot::prelude::drawing::*;

pub fn update_brush(
    action_state: Res<ActionState<DrawingAction>>,
    mut cursor_query: Query<&mut Cursor>,
    brushes: Res<Brushes<DrawingBundle>>,
) {
    for mut cursor in cursor_query.iter_mut() {
        if action_state.just_pressed(DrawingAction::ChangeBrush) {
            cursor.drawing_state.brush_index = brushes.next_index(cursor.drawing_state.brush_index)
        }

        let mut size = cursor.drawing_state.brush_size;

        if action_state.just_pressed(DrawingAction::IncreaseBrush) {
            size += 1;
        } else if action_state.just_pressed(DrawingAction::DecreaseBrush) {
            size -= 1;
        }

        cursor.drawing_state.brush_size = size.clamp(0, 50);
    }
}
