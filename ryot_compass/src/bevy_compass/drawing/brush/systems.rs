use crate::*;
use leafwing_input_manager::action_state::ActionState;

pub fn update_brush(mut cursor_query: Query<(&mut Cursor, &ActionState<DrawingAction>)>) {
    for (mut cursor, action_state) in cursor_query.iter_mut() {
        if action_state.just_pressed(DrawingAction::ChangeBrush) {
            cursor.drawing_state.brush.brush_type = match cursor.drawing_state.brush.brush_type {
                BrushType::Round => BrushType::Square,
                BrushType::Square => BrushType::Diamond,
                BrushType::Diamond => BrushType::Round,
            };
        }

        let mut size = cursor.drawing_state.brush.size;

        if action_state.just_pressed(DrawingAction::IncreaseBrush) {
            size += 1;
        } else if action_state.just_pressed(DrawingAction::DecreaseBrush) {
            size -= 1;
        }

        cursor.drawing_state.brush.size = size.clamp(0, 50);
    }
}
