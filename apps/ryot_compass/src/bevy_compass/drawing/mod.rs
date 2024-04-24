use crate::{gui_is_not_in_use, toggle_grid, CompassAction};
use bevy::prelude::*;
use leafwing_input_manager::common_conditions::*;
use ryot::prelude::{drawing::*, *};

mod commands;
pub use commands::*;

mod draw;
pub use draw::*;

mod delete;
pub use delete::*;

mod undo_redo;
pub use undo_redo::*;

mod brush;
pub use brush::*;

/// The drawing plugin is responsible for handling the core drawing logic and related commands.
/// It is also responsible for keeping track of a command history, used to perform undo/redo actions.
/// The plugin also registers the MapTiles resource, that keeps a map between position and layer in the
/// map and the entity that represents it.
pub struct DrawingPlugin;

impl Plugin for DrawingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommandHistory>()
            .init_resource::<MapTiles<Entity>>()
            .init_resource::<Brushes<DrawingBundle>>()
            .add_plugins(drawing::DrawingPlugin)
            .add_systems(
                Update,
                (
                    on_hold_every!(undo.map(drop), CompassAction::Undo, 100),
                    on_hold_every!(redo.map(drop), CompassAction::Redo, 100),
                    on_hold_every!(handle_drawing_input, CompassAction::Draw, 100),
                    on_hold_every!(toggle_grid, CompassAction::ToggleGrid, 750),
                    on_hold_every!(toggle_deletion, CompassAction::ToggleDeletion, 750),
                    on_hold_every!(change_brush_shape, CompassAction::ChangeBrush, 250),
                    on_hold_every!(change_brush_size(1), CompassAction::IncreaseBrush, 250),
                    on_hold_every!(change_brush_size(-1), CompassAction::DecreaseBrush, 250),
                    set_drawing_input_type.run_if(
                        action_just_released(CompassAction::StartConnectingPoints)
                            .or_else(action_just_pressed(CompassAction::ClearSelection).or_else(
                                action_just_pressed(CompassAction::StartConnectingPoints),
                            )),
                    ),
                    update_drawing_input_type.run_if(action_just_pressed(CompassAction::Draw)),
                )
                    .chain()
                    .run_if(in_state(RyotContentState::Ready))
                    .run_if(gui_is_not_in_use()),
            )
            .add_systems(OnEnter(RyotContentState::Ready), spawn_grid(Color::WHITE));
    }
}
