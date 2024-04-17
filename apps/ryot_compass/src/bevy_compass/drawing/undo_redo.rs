use crate::CommandHistory;
use bevy::prelude::*;

pub fn redo(mut commands: Commands, mut command_history: ResMut<CommandHistory>) -> Option<()> {
    let command_record = command_history.reversed_commands.pop()?;
    commands.add(command_record.command());
    command_history.performed_commands.push(command_record);

    Some(())
}

pub fn undo(mut commands: Commands, mut command_history: ResMut<CommandHistory>) -> Option<()> {
    let command_record = command_history.performed_commands.pop()?;
    commands.add(command_record.reverted());
    command_history.reversed_commands.push(command_record);

    Some(())
}
