use crate::*;
use leafwing_input_manager::action_state::ActionState;

pub fn update_brush(mut cursor_query: Query<(&mut Cursor, &ActionState<DrawingAction>)>) {
    for (mut cursor, action_state) in cursor_query.iter_mut() {
        if action_state.just_pressed(DrawingAction::SetSingleTileBrush) {
            cursor.drawing_state.brush = Brush::SingleTile;
        } else if action_state.just_pressed(DrawingAction::SetRoundBrush) {
            cursor.drawing_state.brush = RoundBrush::default().into();
        } else if action_state.just_pressed(DrawingAction::SetSquareBrush) {
            cursor.drawing_state.brush = SquareBrush::default().into();
        } else if action_state.just_pressed(DrawingAction::SetDiamondBrush) {
            cursor.drawing_state.brush = DiamondBrush::default().into();
        }
    }
}
