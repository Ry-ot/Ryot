use crate::{gui_is_not_in_use, Cursor};
use bevy::app::{App, Plugin, Update};
use bevy::ecs::system::EntityCommand;
use bevy::input::Input;
use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use ryot::bevy_ryot::drawing::{
    DrawingBundle, DrawingCommandHistory, Layer, MapTiles, ReversibleCommandRecord,
    UpdateTileContent,
};
use ryot::bevy_ryot::*;
use ryot::position::TilePosition;
use std::hash::Hash;
use std::marker::PhantomData;

pub struct DrawingPlugin<C: ContentAssets>(PhantomData<C>);

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
            .init_resource::<DrawingCommandHistory>()
            .init_resource::<MapTiles>()
            .register_type::<TilePosition>()
            .register_type::<Layer>()
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
            )
            .add_plugins(ResourceInspectorPlugin::<MapTiles>::default());
    }
}

fn delete_tile_content(
    mut commands: Commands,
    tiles: ResMut<MapTiles>,
    mut command_history: ResMut<DrawingCommandHistory>,
    current_appearance_query: Query<(&mut AppearanceDescriptor, &Visibility), Without<Cursor>>,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_query: Query<(&TilePosition, Changed<TilePosition>), With<Cursor>>,
) {
    for (tile_pos, position_changed) in &cursor_query {
        if !should_react_to_input::<MouseButton>(
            position_changed,
            MouseButton::Right,
            &mouse_button_input,
        ) {
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

        let command = UpdateTileContent(
            None,
            Some(DrawingBundle {
                layer,
                tile_pos,
                appearance,
                visibility: Visibility::Visible,
            }),
        );

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
    keyboard_input: Res<Input<KeyCode>>,
    mut undo_redo_config: ResMut<UndoRedoConfig>,
    mut command_history: ResMut<DrawingCommandHistory>,
) {
    undo_redo_config.timer.tick(time.delta());

    let mut actions: [(
        KeyCode,
        fn(&mut Commands, &ResMut<MapTiles>, &mut ResMut<DrawingCommandHistory>),
    ); 2] = [(KeyCode::U, undo), (KeyCode::R, redo)];

    actions.shuffle(&mut thread_rng());

    for &(key_code, action) in actions.iter() {
        react_to_input(
            key_code,
            action,
            &mut undo_redo_config.timer,
            &keyboard_input,
            &mut commands,
            &tiles,
            &mut command_history,
        );
    }
}

fn draw_to_tile<C: ContentAssets>(
    mut commands: Commands,
    mut tiles: ResMut<MapTiles>,
    mut command_history: ResMut<DrawingCommandHistory>,
    content_assets: Res<C>,
    current_appearance_query: Query<(&mut AppearanceDescriptor, &Visibility), Without<Cursor>>,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_query: Query<
        (&AppearanceDescriptor, &TilePosition, Changed<TilePosition>),
        With<Cursor>,
    >,
) {
    if content_assets.sprite_sheet_data_set().is_none() {
        warn!("Trying to draw a sprite without any loaded content");
        return;
    };

    for (AppearanceDescriptor { group, id, .. }, tile_pos, position_changed) in &cursor_query {
        let Some(prepared_appearance) = content_assets
            .prepared_appearances()
            .get_for_group(*group, *id)
        else {
            return;
        };

        if !should_react_to_input::<MouseButton>(
            position_changed,
            MouseButton::Left,
            &mouse_button_input,
        ) {
            return;
        }

        let tile_pos = *tile_pos;
        let layer = prepared_appearance.layer;
        let appearance = AppearanceDescriptor::new(*group, *id, default());

        let new_bundle = Some(DrawingBundle {
            layer,
            tile_pos,
            appearance,
            visibility: Visibility::Visible,
        });

        let entity = tiles
            .entry(tile_pos)
            .or_default()
            .get(&layer)
            .map_or_else(|| commands.spawn_empty().id(), |&e| e);

        let old_bundle = match current_appearance_query.get(entity) {
            Ok((appearance, visibility)) => Some(DrawingBundle {
                layer,
                tile_pos,
                appearance: *appearance,
                visibility: *visibility,
            }),
            Err(_) => None,
        };

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

fn should_react_to_input<T: Copy + Eq + Hash + Send + Sync + 'static>(
    triggered: bool,
    desired_input: T,
    input_resource: &Res<Input<T>>,
) -> bool {
    if input_resource.just_pressed(desired_input) {
        return true;
    }

    triggered && input_resource.pressed(desired_input)
}

fn redo(
    commands: &mut Commands,
    tiles: &ResMut<MapTiles>,
    command_history: &mut ResMut<DrawingCommandHistory>,
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
    command_history: &mut ResMut<DrawingCommandHistory>,
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

fn react_to_input<T: Copy + Eq + Hash + Send + Sync + 'static>(
    desired_input: T,
    block: fn(&mut Commands, &ResMut<MapTiles>, &mut ResMut<DrawingCommandHistory>),
    timer: &mut Timer,
    keyboard_input: &Res<Input<T>>,
    commands: &mut Commands,
    tiles: &ResMut<MapTiles>,
    command_history: &mut ResMut<DrawingCommandHistory>,
) {
    if should_react_to_input::<T>(timer.just_finished(), desired_input, keyboard_input) {
        timer.reset();
        block(commands, tiles, command_history);
    }
}
