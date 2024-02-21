use crate::CommandHistory;
use bevy::prelude::*;

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

pub fn tick_undo_redo_timer(mut config: ResMut<UndoRedoConfig>, time: Res<Time>) {
    config.timer.tick(time.delta());
}

pub(super) fn redo(
    mut commands: Commands,
    mut undo_redo_config: ResMut<UndoRedoConfig>,
    mut command_history: ResMut<CommandHistory>,
) -> Option<()> {
    let command_record = command_history.reversed_commands.pop()?;
    commands.add(command_record.command());
    command_history.performed_commands.push(command_record);
    undo_redo_config.timer.reset();

    Some(())
}

pub(super) fn undo(
    mut commands: Commands,
    mut undo_redo_config: ResMut<UndoRedoConfig>,
    mut command_history: ResMut<CommandHistory>,
) -> Option<()> {
    let command_record = command_history.performed_commands.pop()?;
    commands.add(command_record.reverted());
    command_history.reversed_commands.push(command_record);
    undo_redo_config.timer.reset();

    Some(())
}
