use crate::helpers::CONTROL_COMMAND;
use crate::MAP_GRAB_INPUTS;
use bevy::app::AppExit;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::{InputKind, Modifier};
use leafwing_input_manager::Actionlike;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            exit.run_if(action_just_pressed(CompassAction::Exit)),
        )
        .add_plugins(InputManagerPlugin::<CompassAction>::default())
        .init_resource::<ActionState<CompassAction>>();
    }
}

fn exit(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}

#[derive(Actionlike, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompassAction {
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
    Exit,
}

impl CompassAction {
    pub fn get_default_input_map() -> InputMap<CompassAction> {
        InputMap::default()
            .insert(CompassAction::Draw, MouseButton::Left)
            .insert_modified(
                CompassAction::ToggleDeletion,
                CONTROL_COMMAND,
                KeyCode::KeyD,
            )
            .insert_modified(CompassAction::Undo, CONTROL_COMMAND, KeyCode::KeyZ)
            .insert_chord(
                CompassAction::Redo,
                [
                    InputKind::Modifier(CONTROL_COMMAND),
                    InputKind::Modifier(Modifier::Shift),
                    InputKind::PhysicalKey(KeyCode::KeyZ),
                ],
            )
            .insert(CompassAction::ChangeBrush, KeyCode::Digit1)
            .insert(CompassAction::ClearSelection, KeyCode::Escape)
            .insert(CompassAction::StartConnectingPoints, Modifier::Shift)
            .insert_modified(
                CompassAction::IncreaseBrush,
                CONTROL_COMMAND,
                KeyCode::Equal,
            )
            .insert_modified(
                CompassAction::DecreaseBrush,
                CONTROL_COMMAND,
                KeyCode::Minus,
            )
            .insert_modified(CompassAction::Exit, CONTROL_COMMAND, KeyCode::KeyQ)
            // Small hack to remove clash with the pancam plugin
            .insert_chord(CompassAction::Stop, MAP_GRAB_INPUTS)
            .build()
    }
}
