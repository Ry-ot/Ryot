use crate::*;
use bevy::utils::HashMap;
use leafwing_input_manager::action_state::ActionState;

pub fn update_brush(mut cursor_query: Query<(&mut Cursor, &ActionState<DrawingAction>)>) {
    let action_to_brush: HashMap<DrawingAction, Brush> = HashMap::from([
        (DrawingAction::SetSingleTileBrush, Brush::SingleTile),
        (
            DrawingAction::SetRoundBrush,
            GeometricBrush::Round(3).into(),
        ),
        (
            DrawingAction::SetSquareBrush,
            GeometricBrush::Square(3).into(),
        ),
        (
            DrawingAction::SetDiamondBrush,
            GeometricBrush::Diamond(3).into(),
        ),
    ]);

    for (mut cursor, action_state) in cursor_query.iter_mut() {
        for (action, brush) in action_to_brush.iter() {
            if action_state.just_pressed(*action) {
                cursor.drawing_state.brush = *brush;
                break;
            }
        }

        if action_state.just_pressed(DrawingAction::IncreaseBrush) {
            cursor.drawing_state.brush.increase();
        } else if action_state.just_pressed(DrawingAction::DecreaseBrush) {
            cursor.drawing_state.brush.decrease();
        }
    }
}
