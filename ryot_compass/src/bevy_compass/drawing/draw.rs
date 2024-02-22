use crate::{
    delete_top_most_elements_in_positions, CommandHistory, Cursor, DrawingAction, InputType,
    ToolMode,
};
use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use ryot::bevy_ryot::map::MapTiles;
use ryot::bevy_ryot::*;
use ryot::prelude::{drawing::*, position::*};
use ryot::Layer;

pub fn draw_on_hold<C: ContentAssets>() -> SystemConfigs {
    on_hold(
        handle_drawing_input::<C, Changed<TilePosition>>,
        DrawingAction::Draw,
    )
}

pub fn draw_on_click<C: ContentAssets>() -> SystemConfigs {
    on_press(handle_drawing_input::<C, ()>, DrawingAction::Draw)
}

/// System responsible for handling the drawing inputs. Drawing inputs can be a multitude of things,
/// such as drawing, erasing, selecting, etc. In our context, we are only handling drawing and erasing,
/// keeping the possibility of adding more tools in the future (e.g. if we want special tools for marking
/// some protection zone in the map, for drawing paths, creating areas/cities, etc).
fn handle_drawing_input<C: ContentAssets, F: ReadOnlyWorldQuery>(
    mut commands: Commands,
    mut tiles: ResMut<MapTiles>,
    mut command_history: ResMut<CommandHistory>,
    content_assets: Res<C>,
    brushes: Res<Brushes<DrawingBundle>>,
    q_current_appearance: Query<(&Visibility, &Layer, &AppearanceDescriptor), With<TileComponent>>,
    cursor_query: Query<(&AppearanceDescriptor, &TilePosition, &Cursor), F>,
) {
    get_cursor_inputs(
        &content_assets,
        &brushes,
        &cursor_query,
        |cursor: &Cursor, bundles: Vec<DrawingBundle>| match cursor.drawing_state.tool_mode {
            ToolMode::Draw => {
                create_or_update_content_for_positions(
                    &bundles,
                    &mut commands,
                    &mut command_history,
                    &mut tiles,
                    &q_current_appearance,
                );
            }
            ToolMode::Erase => {
                delete_top_most_elements_in_positions(
                    &bundles,
                    &mut commands,
                    &mut command_history,
                    &tiles,
                    &q_current_appearance,
                );
            }
        },
    );
}

fn get_cursor_inputs<C: ContentAssets, F: ReadOnlyWorldQuery>(
    content_assets: &Res<C>,
    brushes: &Res<Brushes<DrawingBundle>>,
    cursor_query: &Query<(&AppearanceDescriptor, &TilePosition, &Cursor), F>,
    mut callback: impl FnMut(&Cursor, Vec<DrawingBundle>),
) {
    if content_assets.sprite_sheet_data_set().is_none() {
        warn!("Trying to draw a sprite without any loaded content");
        return;
    };

    for (appearance, tile_pos, cursor) in cursor_query {
        if !cursor.drawing_state.enabled {
            continue;
        }

        let Some(prepared_appearance) = content_assets
            .prepared_appearances()
            .get_for_group(appearance.group, appearance.id)
        else {
            continue;
        };

        let layer = prepared_appearance.layer;
        let appearance = AppearanceDescriptor::new(appearance.group, appearance.id, default());

        callback(
            cursor,
            brushes(
                cursor.drawing_state.brush_index,
                cursor.drawing_state.input_type.into(),
                DrawingBundle::new(layer, *tile_pos, appearance),
            ),
        );
    }
}

fn create_or_update_content_for_positions(
    to_draw: &[DrawingBundle],
    commands: &mut Commands,
    command_history: &mut ResMut<CommandHistory>,
    tiles: &mut ResMut<MapTiles>,
    q_current_appearance: &Query<(&Visibility, &Layer, &AppearanceDescriptor), With<TileComponent>>,
) {
    let mut old_info: Vec<DrawingInfo> = vec![];
    let mut to_draw = to_draw.to_vec();

    for new_bundle in to_draw.iter_mut() {
        let current_info = get_current_info(*new_bundle, tiles, q_current_appearance);

        if let Layer::Bottom(mut bottom) = current_info.1 {
            if let Some(_) = current_info.3 {
                new_bundle.layer = Layer::Bottom(bottom.next().unwrap_or(bottom));
            }
        }

        old_info.push(current_info);
    }

    let new_info = to_draw
        .iter()
        .copied()
        .map(|bundle| bundle.into())
        .collect::<Vec<DrawingInfo>>();

    if new_info
        .iter()
        .filter_map(|info| Some(info.3?.id))
        .eq(old_info.iter().filter_map(|info| Some(info.3?.id)))
    {
        return;
    }

    let command = UpdateTileContent::new(new_info, old_info);

    commands.add(command.clone());
    command_history.reversed_commands.clear();
    command_history.performed_commands.push(command.into());
}

pub fn get_current_info(
    new_bundle: DrawingBundle,
    tiles: &mut ResMut<MapTiles>,
    q_current_appearance: &Query<(&Visibility, &Layer, &AppearanceDescriptor), With<TileComponent>>,
) -> DrawingInfo {
    let mut old_info = (
        new_bundle.tile_pos,
        new_bundle.layer,
        new_bundle.visibility,
        None,
    );

    let Some(tile) = tiles.get(&new_bundle.tile_pos) else {
        return old_info;
    };

    let entity = match new_bundle.layer {
        Layer::Bottom(mut bottom) => {
            let mut entity = None;

            loop {
                let layer = Layer::Bottom(bottom);
                let next_entity = tile.peek_for_layer(layer);

                if next_entity.is_none() {
                    break;
                }

                let Ok((visibility, ..)) = q_current_appearance.get(next_entity.unwrap()) else {
                    break;
                };

                if visibility == Visibility::Hidden {
                    break;
                }

                old_info.1 = layer;
                entity = next_entity;

                if bottom.next().is_none() {
                    break;
                }

                bottom = bottom.next().unwrap();
            }

            entity
        }
        layer => tile.peek_for_layer(layer),
    };

    let Some(entity) = entity else {
        return old_info;
    };

    old_info.3 = match q_current_appearance.get(entity) {
        Ok((visibility, _, appearance)) if visibility != Visibility::Hidden => Some(*appearance),
        _ => None,
    };

    old_info
}

pub fn get_next_bottom_layer_stack(
    new_bundle: DrawingBundle,
    tiles: &mut ResMut<MapTiles>,
) -> Option<Layer> {
    let Some(_) = tiles
        .entry(new_bundle.tile_pos)
        .or_default()
        .peek_for_layer(new_bundle.layer)
    else {
        return Some(new_bundle.layer);
    };

    match new_bundle.layer {
        Layer::Bottom(mut bottom) => match bottom.next() {
            Some(bottom) => Some(Layer::Bottom(bottom)),
            _ => None,
        },
        _ => None,
    }
}

pub fn update_drawing_input_type(mut cursor_query: Query<(&TilePosition, &mut Cursor)>) {
    for (cursor_pos, mut cursor) in &mut cursor_query {
        cursor.drawing_state.input_type = match cursor.drawing_state.input_type {
            InputType::DoubleClick(_) => InputType::DoubleClick(Some(*cursor_pos)),
            input_type => input_type,
        };
    }
}

pub fn set_drawing_input_type(
    mut previous_size: Local<i32>,
    mut cursor_query: Query<&mut Cursor>,
    action_state: Res<ActionState<DrawingAction>>,
) {
    for mut cursor in &mut cursor_query {
        if let InputType::SingleClick(size) = cursor.drawing_state.input_type {
            *previous_size = size;
        }

        if action_state.just_pressed(&DrawingAction::StartConnectingPoints) {
            cursor.drawing_state.input_type = InputType::DoubleClick(None);
        } else {
            cursor.drawing_state.input_type = InputType::SingleClick(*previous_size);
        }
    }
}
