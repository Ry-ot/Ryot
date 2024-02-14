use crate::DrawingAction;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use ryot::bevy_ryot::map::MapTiles;
use ryot::bevy_ryot::*;
use ryot::prelude::drawing::*;
use std::ops::Deref;

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
    on_hold(undo_tile_action, DrawingAction::Undo).run_if(time_is_finished())
}

pub fn undo_on_click() -> SystemConfigs {
    on_press(undo_tile_action, DrawingAction::Undo)
}

pub fn redo_on_hold() -> SystemConfigs {
    on_hold(redo_tile_action, DrawingAction::Redo).run_if(time_is_finished())
}

pub fn redo_on_click() -> SystemConfigs {
    on_press(redo_tile_action, DrawingAction::Redo)
}

pub fn tick_undo_redo_timer(mut config: ResMut<UndoRedoConfig>, time: Res<Time>) {
    config.timer.tick(time.delta());
}

pub(super) fn redo_tile_action(
    mut commands: Commands,
    tiles: ResMut<MapTiles>,
    mut undo_redo_config: ResMut<UndoRedoConfig>,
    mut command_history: ResMut<CommandHistory>,
) {
    if redo(&mut commands, &tiles, &mut command_history) {
        undo_redo_config.timer.reset();
    }
}

pub(super) fn undo_tile_action(
    mut commands: Commands,
    tiles: ResMut<MapTiles>,
    mut undo_redo_config: ResMut<UndoRedoConfig>,
    mut command_history: ResMut<CommandHistory>,
) {
    if undo(&mut commands, &tiles, &mut command_history) {
        undo_redo_config.timer.reset();
    }
}

fn redo(
    commands: &mut Commands,
    tiles: &ResMut<MapTiles>,
    command_history: &mut ResMut<CommandHistory>,
) -> bool {
    if let Some(command_record) = command_history.reversed_commands.pop() {
        match &command_record {
            CommandType::TileCommand(command_record) => command_record.command.redo(
                commands,
                get_entity_from_command_record(tiles, command_record),
            ),
            CommandType::Batch(batch_size) => {
                for _ in 0..*batch_size.deref() {
                    redo(commands, tiles, command_history);
                }
            }
        }

        command_history.performed_commands.push(command_record);

        return true;
    }

    false
}

fn undo(
    commands: &mut Commands,
    tiles: &ResMut<MapTiles>,
    command_history: &mut ResMut<CommandHistory>,
) -> bool {
    if let Some(command_record) = command_history.performed_commands.pop() {
        match &command_record {
            CommandType::TileCommand(command_record) => command_record.command.undo(
                commands,
                get_entity_from_command_record(tiles, command_record),
            ),
            CommandType::Batch(batch_size) => {
                for _ in 0..*batch_size.deref() {
                    undo(commands, tiles, command_history);
                }
            }
        }

        command_history.reversed_commands.push(command_record);

        return true;
    }

    false
}

fn get_entity_from_command_record(
    tiles: &ResMut<MapTiles>,
    command_record: &TileCommandRecord,
) -> Option<Entity> {
    tiles
        .get(&command_record.tile_pos)
        .and_then(|t| t.get(&command_record.layer))
        .copied()
}
