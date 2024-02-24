use crate::{gui_is_not_in_use, helpers::CONTROL_COMMAND, MAP_GRAB_INPUTS};
use bevy::prelude::*;
use leafwing_input_manager::user_input::InputKind;
use leafwing_input_manager::{common_conditions::*, prelude::*};
use ryot::bevy_ryot::map::MapTiles;
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

#[derive(Actionlike, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrawingAction {
    Stop,
    Draw,
    ToggleDeletion,
    Undo,
    Redo,
    StartConnectingPoints,
    ChangeBrush,
    IncreaseBrush,
    DecreaseBrush,
    ClearSelection,
}

impl DrawingAction {
    pub fn get_default_input_map() -> InputMap<DrawingAction> {
        InputMap::default()
            .insert(DrawingAction::Draw, MouseButton::Left)
            .insert_modified(
                DrawingAction::ToggleDeletion,
                CONTROL_COMMAND,
                KeyCode::KeyD,
            )
            .insert_modified(DrawingAction::Undo, CONTROL_COMMAND, KeyCode::KeyZ)
            .insert_chord(
                DrawingAction::Redo,
                [
                    InputKind::Modifier(CONTROL_COMMAND),
                    InputKind::Modifier(Modifier::Shift),
                    InputKind::PhysicalKey(KeyCode::KeyZ),
                ],
            )
            .insert(DrawingAction::ChangeBrush, KeyCode::Digit1)
            .insert(DrawingAction::ClearSelection, KeyCode::Escape)
            .insert(DrawingAction::StartConnectingPoints, Modifier::Shift)
            .insert_modified(
                DrawingAction::IncreaseBrush,
                CONTROL_COMMAND,
                KeyCode::Equal,
            )
            .insert_modified(
                DrawingAction::DecreaseBrush,
                CONTROL_COMMAND,
                KeyCode::Minus,
            )
            // Small hack to remove clash with the pancam plugin
            .insert_chord(DrawingAction::Stop, MAP_GRAB_INPUTS)
            .build()
    }
}

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
            .add_plugins(InputManagerPlugin::<DrawingAction>::default())
            .init_resource::<ActionState<DrawingAction>>()
            .add_systems(
                Update,
                (
                    set_drawing_input_type.run_if(
                        action_just_released(DrawingAction::StartConnectingPoints)
                            .or_else(action_just_pressed(DrawingAction::ClearSelection).or_else(
                                action_just_pressed(DrawingAction::StartConnectingPoints),
                            )),
                    ),
                    (draw_on_click::<C>(), draw_on_hold::<C>()),
                    toggle_deletion.run_if(action_just_pressed(DrawingAction::ToggleDeletion)),
                    (
                        on_press(undo.map(drop), DrawingAction::Undo),
                        on_hold(undo.map(drop), DrawingAction::Undo).run_if(run_every_millis(300)),
                        on_press(redo.map(drop), DrawingAction::Redo),
                        on_hold(redo.map(drop), DrawingAction::Redo).run_if(run_every_millis(300)),
                    )
                        .chain(),
                    (
                        change_brush_shape.run_if(action_just_pressed(DrawingAction::ChangeBrush)),
                        change_brush_size(1)
                            .run_if(action_just_pressed(DrawingAction::IncreaseBrush)),
                        change_brush_size(-1)
                            .run_if(action_just_pressed(DrawingAction::DecreaseBrush)),
                    ),
                    update_drawing_input_type.run_if(action_just_pressed(DrawingAction::Draw)),
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
