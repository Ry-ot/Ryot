use crate::DrawingAction;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use ryot::bevy_ryot::*;
use ryot::prelude::drawing::*;

/// This resource is used to configure the undo/redo system.
/// Currently, it only contains a timer that is used to control the speed of the undo/redo actions.
/// The default cooldown for undo/redo is 100ms.
#[derive(Resource)]
pub struct UndoRedoConfig {
    timer: Timer,
}

impl Default for UndoRedoConfig {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

pub fn time_is_finished() -> impl FnMut(Res<UndoRedoConfig>) -> bool {
    move |config: Res<UndoRedoConfig>| config.timer.just_finished()
}

pub fn undo_on_hold() -> SystemConfigs {
    on_hold(undo, DrawingAction::Undo).run_if(time_is_finished())
}

pub fn undo_on_click() -> SystemConfigs {
    on_press(undo, DrawingAction::Undo)
}

pub fn redo_on_hold() -> SystemConfigs {
    on_hold(redo, DrawingAction::Redo).run_if(time_is_finished())
}

pub fn redo_on_click() -> SystemConfigs {
    on_press(redo, DrawingAction::Redo)
}

pub fn tick_undo_redo_timer(mut config: ResMut<UndoRedoConfig>, time: Res<Time>) {
    config.timer.tick(time.delta());
}

pub(super) fn redo(
    mut commands: Commands,
    mut undo_redo_config: ResMut<UndoRedoConfig>,
    mut command_history: ResMut<CommandHistory>,
) {
    if let Some(command_record) = command_history.reversed_commands.pop() {
        command_record.redo(&mut commands);
        command_history.performed_commands.push(command_record);
        undo_redo_config.timer.reset();
    }
}

pub(super) fn undo(
    mut commands: Commands,
    mut undo_redo_config: ResMut<UndoRedoConfig>,
    mut command_history: ResMut<CommandHistory>,
) {
    if let Some(command_record) = command_history.performed_commands.pop() {
        command_record.undo(&mut commands);
        command_history.reversed_commands.push(command_record);
        undo_redo_config.timer.reset();
    }
}
