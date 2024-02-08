use crate::{gui_is_not_in_use, Cursor};
use bevy::app::{App, Plugin, Update};
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use ryot::bevy_ryot::drawing::{
    CommandHistory, DrawingBundle, Layer, MapTiles, ReversibleCommandRecord, UpdateTileContent,
};
use ryot::bevy_ryot::*;
use ryot::position::TilePosition;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Actionlike, Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrawingAction {
    Stop,
    Draw,
    Erase,
    Undo,
    Redo,
}

impl DrawingAction {
    pub fn get_default_input_map() -> InputMap<DrawingAction> {
        let mut input_map = InputMap::new([
            (MouseButton::Left, DrawingAction::Draw),
            (MouseButton::Right, DrawingAction::Erase),
        ]);

        input_map.insert_chord([KeyCode::ControlLeft, KeyCode::Z], DrawingAction::Undo);
        input_map.insert_chord([KeyCode::ControlLeft, KeyCode::R], DrawingAction::Redo);

        // Small hack to remove clash with the pancam plugin
        input_map.insert_chord(
            [
                InputKind::Mouse(MouseButton::Left),
                InputKind::Keyboard(KeyCode::AltLeft),
            ],
            DrawingAction::Stop,
        );
        input_map.insert_chord(
            [
                InputKind::Mouse(MouseButton::Left),
                InputKind::Keyboard(KeyCode::AltRight),
            ],
            DrawingAction::Stop,
        );

        input_map
    }
}

/// This resource is used to configure the undo/redo system.
/// Currently, it only contains a timer that is used to control the speed of the undo/redo actions.
/// The default cooldown for undo/redo is 100ms.
#[derive(Resource)]
struct UndoRedoConfig {
    timer: Timer,
}

impl Default for UndoRedoConfig {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
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
        app.init_resource::<UndoRedoConfig>()
            .init_resource::<CommandHistory>()
            .init_resource::<MapTiles>()
            .add_plugins(drawing::DrawingPlugin)
            .add_plugins(InputManagerPlugin::<DrawingAction>::default())
            .add_systems(
                Update,
                (
                    draw_to_tile::<C>,
                    delete_tile_content,
                    undo_redo_tile_action,
                )
                    .run_if(in_state(InternalContentState::Ready))
                    .run_if(gui_is_not_in_use()),
            )
            .add_systems(
                OnExit(InternalContentState::PreparingSprites),
                spawn_grid(Color::WHITE),
            );
    }
}

/// A function that listens to the right mouse button and deletes the content of the tile under the cursor.
/// It always delete the topmost content of the tile, following the Z-ordering.
fn delete_tile_content(
    mut commands: Commands,
    tiles: ResMut<MapTiles>,
    mut command_history: ResMut<CommandHistory>,
    current_appearance_query: Query<(&mut AppearanceDescriptor, &Visibility), Without<Cursor>>,
    cursor_query: Query<
        (
            &ActionState<DrawingAction>,
            &TilePosition,
            Changed<TilePosition>,
        ),
        With<Cursor>,
    >,
) {
    for (action_state, tile_pos, position_changed) in &cursor_query {
        if !check_action(DrawingAction::Erase, position_changed, action_state) {
            return;
        }

        let tile_pos = *tile_pos;

        let Some(tile_content) = tiles.get(&tile_pos) else {
            return;
        };

        let mut content: Option<(Entity, Layer, AppearanceDescriptor)> = None;

        for layer in [Layer::Top, Layer::Items, Layer::Bottom, Layer::Ground] {
            if let Some(entity) = tile_content.get(&layer) {
                if let Ok((current, visibility)) = current_appearance_query.get(*entity) {
                    if visibility == Visibility::Hidden {
                        continue;
                    }

                    content = Some((*entity, layer, *current));
                    break;
                }
            }
        }

        let Some((entity, layer, appearance)) = content else {
            return;
        };

        let command =
            UpdateTileContent(None, Some(DrawingBundle::new(layer, tile_pos, appearance)));

        commands.add(command.with_entity(entity));
        command_history
            .performed_commands
            .push(ReversibleCommandRecord::new(
                layer,
                tile_pos,
                Box::new(command),
            ));
    }
}

fn undo_redo_tile_action(
    time: Res<Time>,
    mut commands: Commands,
    tiles: ResMut<MapTiles>,
    mut undo_redo_config: ResMut<UndoRedoConfig>,
    mut command_history: ResMut<CommandHistory>,
    action_state_query: Query<&ActionState<DrawingAction>, With<Cursor>>,
) {
    undo_redo_config.timer.tick(time.delta());

    let action_state = action_state_query.single();
    let mut actions = [DrawingAction::Undo, DrawingAction::Redo];
    actions.shuffle(&mut thread_rng());

    for action in actions {
        let is_timer_finished = undo_redo_config.timer.just_finished();

        if check_action(action, is_timer_finished, action_state) {
            undo_redo_config.timer.reset();
            match action {
                DrawingAction::Undo => undo(&mut commands, &tiles, &mut command_history),
                DrawingAction::Redo => redo(&mut commands, &tiles, &mut command_history),
                _ => (),
            }
        }
    }
}

fn draw_to_tile<C: ContentAssets>(
    mut commands: Commands,
    mut tiles: ResMut<MapTiles>,
    mut command_history: ResMut<CommandHistory>,
    content_assets: Res<C>,
    current_appearance_query: Query<(&mut AppearanceDescriptor, &Visibility), Without<Cursor>>,
    cursor_query: Query<(
        &ActionState<DrawingAction>,
        &AppearanceDescriptor,
        &TilePosition,
        &Cursor,
        Changed<TilePosition>,
    )>,
) {
    if content_assets.sprite_sheet_data_set().is_none() {
        warn!("Trying to draw a sprite without any loaded content");
        return;
    };

    for (
        action_state,
        AppearanceDescriptor { group, id, .. },
        tile_pos,
        cursor,
        position_changed,
    ) in &cursor_query
    {
        if !cursor.drawing_state.enabled {
            continue;
        }

        let Some(prepared_appearance) = content_assets
            .prepared_appearances()
            .get_for_group(*group, *id)
        else {
            return;
        };

        if !check_action(DrawingAction::Draw, position_changed, action_state) {
            return;
        }

        let tile_pos = *tile_pos;
        let layer = prepared_appearance.layer;
        let appearance = AppearanceDescriptor::new(*group, *id, default());

        let entity = tiles
            .entry(tile_pos)
            .or_default()
            .get(&layer)
            .map_or_else(|| commands.spawn_empty().id(), |&e| e);

        let old_bundle = match current_appearance_query.get(entity) {
            Ok((appearance, visibility)) => {
                Some(DrawingBundle::new(layer, tile_pos, *appearance).with_visibility(*visibility))
            }
            Err(_) => None,
        };

        let new_bundle = Some(DrawingBundle::new(layer, tile_pos, appearance));

        if old_bundle == new_bundle {
            return;
        }

        let command = UpdateTileContent(new_bundle, old_bundle);
        commands.add(command.with_entity(entity));
        command_history
            .performed_commands
            .push(ReversibleCommandRecord::new(
                layer,
                tile_pos,
                Box::new(command),
            ));
    }
}

fn redo(
    commands: &mut Commands,
    tiles: &ResMut<MapTiles>,
    command_history: &mut ResMut<CommandHistory>,
) {
    if let Some(command_record) = command_history.reversed_commands.pop() {
        command_record.command.redo(
            commands,
            get_entity_from_command_record(tiles, &command_record),
        );
        command_history.performed_commands.push(command_record);
    }
}

fn undo(
    commands: &mut Commands,
    tiles: &ResMut<MapTiles>,
    command_history: &mut ResMut<CommandHistory>,
) {
    if let Some(command_record) = command_history.performed_commands.pop() {
        command_record.command.undo(
            commands,
            get_entity_from_command_record(tiles, &command_record),
        );
        command_history.reversed_commands.push(command_record);
    }
}

fn get_entity_from_command_record(
    tiles: &ResMut<MapTiles>,
    command_record: &ReversibleCommandRecord,
) -> Option<Entity> {
    tiles
        .get(&command_record.tile_pos)
        .and_then(|t| t.get(&command_record.layer))
        .copied()
}
