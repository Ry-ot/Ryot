use crate::*;
use leafwing_input_manager::action_state::ActionState;
use ryot::bevy_ryot::drawing::DrawingBundle;

pub fn update_brush(
    mut cursor_query: Query<(&mut Cursor, &ActionState<DrawingAction>)>,
    brushes: Res<Brushes<DrawingBundle>>,
) {
    for (mut cursor, action_state) in cursor_query.iter_mut() {
        if action_state.just_pressed(DrawingAction::ChangeBrush) {
            cursor.drawing_state.brush_index += 1;

            if cursor.drawing_state.brush_index >= brushes.len() {
                cursor.drawing_state.brush_index = 0;
            }
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
