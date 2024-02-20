use crate::{Cursor, InputType};
use bevy::prelude::*;
use ryot::bevy_ryot::drawing::DrawingBundle;
use ryot::prelude::drawing::*;

pub fn change_brush_shape(
    mut cursor_query: Query<&mut Cursor>,
    brushes: Res<Brushes<DrawingBundle>>,
) {
    for mut cursor in cursor_query.iter_mut() {
        cursor.drawing_state.brush_index = brushes.next_index(cursor.drawing_state.brush_index)
    }
}

pub fn change_brush_size(delta: i32) -> impl FnMut(Query<&mut Cursor>) {
    move |mut cursor_query: Query<&mut Cursor>| {
        for mut cursor in cursor_query.iter_mut() {
            if let InputType::SingleClick(size) = &mut cursor.drawing_state.input_type {
                *size = (*size + delta).clamp(0, 50);
            }
        }
    }
}
