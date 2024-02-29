use crate::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CursorEvents>();
    }
}

#[derive(Event, Clone, Copy, Debug)]
pub enum CursorEvents {
    BrushChanged(usize),
    ToolModeChanged(ToolMode),
    InputTypeChanged(InputType),
    SizeChanged(i32),
}
