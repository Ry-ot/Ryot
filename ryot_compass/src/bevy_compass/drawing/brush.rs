use crate::{Cursor, CursorEvents, InputType};
use bevy::prelude::*;
use ryot::bevy_ryot::drawing::DrawingBundle;
use ryot::prelude::drawing::*;

pub fn change_brush_shape(
    cursor_query: Query<&Cursor>,
    brushes: Res<Brushes<DrawingBundle>>,
    mut cursor_events_writer: EventWriter<CursorEvents>,
) {
    for cursor in cursor_query.iter() {
        let next_brush = brushes.next_index(cursor.drawing_state.brush_index);

        if next_brush != cursor.drawing_state.brush_index {
            cursor_events_writer.send(CursorEvents::BrushChanged(next_brush));
        }
    }
}

pub fn change_brush_size(delta: i32) -> impl FnMut(Query<&Cursor>, EventWriter<CursorEvents>) {
    move |cursor_query: Query<&Cursor>, mut cursor_events_writer: EventWriter<CursorEvents>| {
        for cursor in cursor_query.iter() {
            if let InputType::SingleClick(size) = cursor.drawing_state.input_type {
                cursor_events_writer.send(CursorEvents::SizeChanged(size + delta));
            }
        }
    }
}
