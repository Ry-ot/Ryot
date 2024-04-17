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
    ToggleGrid,
    Undo,
    Redo,
    StartConnectingPoints,
    ChangeBrush,
    IncreaseBrush,
    DecreaseBrush,
    ClearSelection,
    Focus,
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
            .insert_modified(CompassAction::ToggleGrid, CONTROL_COMMAND, KeyCode::KeyG)
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
            .insert(CompassAction::Stop, MAP_GRAB_INPUTS)
            .insert(CompassAction::Focus, MouseButton::Middle)
            .insert_modified(CompassAction::Focus, Modifier::Alt, MouseButton::Left)
            .build()
    }

    pub fn get_hotkeys_list() -> Vec<String> {
        #[cfg(target_os = "macos")]
        let control_key = "Cmd";
        #[cfg(not(target_os = "macos"))]
        let control_key = "Ctrl";
        let shift_key = "Shift";

        vec![
            "Zoom (Scroll)".to_string(),
            "Draw (Left Click)".to_string(),
            format!("Draw Connecting Points (Hold {})", shift_key),
            "Move map (Right Click)".to_string(),
            "Move to Cursor (Middle Click or Alt + Left Click)".to_string(),
            format!("Toggle Grid ({} G)", control_key),
            format!("Toggle Deletion ({} D)", control_key),
            format!("Undo ({} Z)", control_key),
            format!("Redo ({} {} Z)", control_key, shift_key),
            "Change Brush (1)".to_string(),
            "Clear Selection (Esc)".to_string(),
            format!("Increase Brush ({} +/=)", control_key),
            format!("Decrease Brush ({} -)", control_key),
            format!("Exit ({} Q)", control_key),
        ]
    }
}
