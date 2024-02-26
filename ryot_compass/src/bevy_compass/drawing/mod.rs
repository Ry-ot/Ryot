use crate::{gui_is_not_in_use, toggle_grid, CompassAction};
use bevy::prelude::*;
use leafwing_input_manager::common_conditions::*;
use ryot::bevy_ryot::map::MapTiles;
use ryot::input_action;
use ryot::prelude::{drawing::*, *};
use std::marker::PhantomData;

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
pub struct DrawingPlugin<C: ContentAssets>(PhantomData<C>);

impl<C: ContentAssets> DrawingPlugin<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C: ContentAssets> Default for DrawingPlugin<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: ContentAssets> Plugin for DrawingPlugin<C> {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommandHistory>()
            .init_resource::<MapTiles>()
            .init_resource::<Brushes<DrawingBundle>>()
            .add_plugins(drawing::DrawingPlugin)
            .add_systems(
                Update,
                (
                    set_drawing_input_type.run_if(
                        action_just_released(CompassAction::StartConnectingPoints)
                            .or_else(action_just_pressed(CompassAction::ClearSelection).or_else(
                                action_just_pressed(CompassAction::StartConnectingPoints),
                            )),
                    ),
                    input_action!(handle_drawing_input::<C>, CompassAction::Draw, 50),
                    input_action!(toggle_deletion, CompassAction::ToggleDeletion, 750),
                    input_action!(toggle_grid, CompassAction::ToggleGrid, 750),
                    input_action!(undo.map(drop), CompassAction::Undo, 50),
                    input_action!(redo.map(drop), CompassAction::Redo, 50),
                    (
                        change_brush_shape.run_if(action_just_pressed(CompassAction::ChangeBrush)),
                        change_brush_size(1)
                            .run_if(action_just_pressed(CompassAction::IncreaseBrush)),
                        change_brush_size(-1)
                            .run_if(action_just_pressed(CompassAction::DecreaseBrush)),
                    ),
                    update_drawing_input_type.run_if(action_just_pressed(CompassAction::Draw)),
                )
                    .chain()
                    .run_if(in_state(InternalContentState::Ready))
                    .run_if(gui_is_not_in_use()),
            )
            .add_systems(
                OnExit(InternalContentState::PreparingSprites),
                spawn_grid::<C>(Color::WHITE),
            );
    }
}
