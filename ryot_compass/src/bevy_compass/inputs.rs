use crate::helpers::CONTROL_COMMAND;
use crate::MAP_GRAB_INPUTS;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::{InputKind, Modifier};
use leafwing_input_manager::Actionlike;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<DrawingAction>::default())
            .init_resource::<ActionState<DrawingAction>>();
    }
}

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
