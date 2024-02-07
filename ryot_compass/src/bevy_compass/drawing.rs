use crate::{gui_is_not_in_use, Cursor};
use bevy::app::{App, Plugin, Update};
use bevy::ecs::system::EntityCommand;
use bevy::input::Input;
use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
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
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
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
                (draw_to_tile::<C>, delete_tile_content, undo_tile_action)
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

fn undo_tile_action(
    time: Res<Time>,
    mut commands: Commands,
    tiles: ResMut<MapTiles>,
    keyboard_input: Res<Input<KeyCode>>,
    mut undo_redo_config: ResMut<UndoRedoConfig>,
    mut command_history: ResMut<DrawingCommandHistory>,
) {
    undo_redo_config.timer.tick(time.delta());
    if !undo_redo_config.timer.just_finished() {
        return;
    }

    let fn_get_entity = |command_record: &ReversibleCommandRecord| {
        tiles
            .get(&command_record.tile_pos)
            .and_then(|t| t.get(&command_record.layer))
            .copied()
    };

    if should_react_to_input::<KeyCode>(true, KeyCode::U, &keyboard_input) {
        if let Some(command_record) = command_history.performed_commands.pop() {
            command_record
                .command
                .undo(&mut commands, fn_get_entity(&command_record));
            command_history.reversed_commands.push(command_record);
        }
    }

    if should_react_to_input::<KeyCode>(true, KeyCode::R, &keyboard_input) {
        if let Some(command_record) = command_history.reversed_commands.pop() {
            command_record
                .command
                .redo(&mut commands, fn_get_entity(&command_record));
            command_history.performed_commands.push(command_record);
        }
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
