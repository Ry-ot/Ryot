use crate::DrawingAction;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use ryot::bevy_ryot::*;
use ryot::prelude::drawing::*;
use std::ops::Deref;

/// This resource is used to configure the undo/redo system.
/// Currently, it only contains a timer that is used to control the speed of the undo/redo actions.
/// The default cooldown for undo/redo is 100ms.
#[derive(Resource)]
pub(super) struct UndoRedoConfig {
    timer: Timer,
}

impl Default for UndoRedoConfig {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

pub(super) fn undo_redo_tile_action(
    time: Res<Time>,
    mut commands: Commands,
    tiles: ResMut<MapTiles>,
    mut undo_redo_config: ResMut<UndoRedoConfig>,
    mut command_history: ResMut<CommandHistory>,
    action_state: Res<ActionState<DrawingAction>>,
) {
    undo_redo_config.timer.tick(time.delta());

    let mut actions = [DrawingAction::Undo, DrawingAction::Redo];
    actions.shuffle(&mut thread_rng());

    for action in actions {
        let is_timer_finished = undo_redo_config.timer.just_finished();

        if check_action(action, is_timer_finished, &action_state) {
            undo_redo_config.timer.reset();
            match action {
                DrawingAction::Undo => undo(&mut commands, &tiles, &mut command_history),
                DrawingAction::Redo => redo(&mut commands, &tiles, &mut command_history),
                _ => (),
            }
        }
    }
}

fn redo(
    commands: &mut Commands,
    tiles: &ResMut<MapTiles>,
    command_history: &mut ResMut<CommandHistory>,
) {
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
    }
}

fn undo(
    commands: &mut Commands,
    tiles: &ResMut<MapTiles>,
    command_history: &mut ResMut<CommandHistory>,
) {
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
    }
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
