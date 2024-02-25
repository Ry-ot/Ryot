use crate::{
    delete_top_most_elements_in_positions, CommandHistory, CompassAction, Cursor, InputType,
    ToolMode,
};
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use ryot::bevy_ryot::map::MapTiles;
use ryot::bevy_ryot::*;
use ryot::prelude::{drawing::*, position::*};
use ryot::Layer;

/// System responsible for handling the drawing inputs. Drawing inputs can be a multitude of things,
/// such as drawing, erasing, selecting, etc. In our context, we are only handling drawing and erasing,
/// keeping the possibility of adding more tools in the future (e.g. if we want special tools for marking
/// some protection zone in the map, for drawing paths, creating areas/cities, etc).
pub fn handle_drawing_input<C: ContentAssets>(
    mut commands: Commands,
    mut tiles: ResMut<MapTiles>,
    mut command_history: ResMut<CommandHistory>,
    content_assets: Res<C>,
    brushes: Res<Brushes<DrawingBundle>>,
    q_current_appearance: Query<(&Visibility, &Layer, &AppearanceDescriptor), With<TileComponent>>,
    cursor_query: Query<(&AppearanceDescriptor, &TilePosition, &Cursor)>,
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

fn get_cursor_inputs<C: ContentAssets, F: QueryFilter>(
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
        let mut appearance = get_current_appearance(*new_bundle, tiles, q_current_appearance);

        // If an old appearance was found, it is only pertinent if the layer doesn't change.
        // If the layer changes, the old state is not relevant for a new layer.
        // The old state of a new layer is empty, like a new entity being added, so appearance
        // is set to None. This guarantees that reversion works properly in-between layers.
        if appearance.is_some() {
            if let Layer::Bottom(mut bottom) = new_bundle.layer {
                new_bundle.layer = Layer::Bottom(bottom.next().unwrap_or(bottom));
                appearance = None;
            }
        }

        old_info.push((
            new_bundle.tile_pos,
            new_bundle.layer,
            new_bundle.visibility,
            appearance,
        ));
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

pub fn get_current_appearance(
    new_bundle: DrawingBundle,
    tiles: &mut ResMut<MapTiles>,
    q_current_appearance: &Query<(&Visibility, &Layer, &AppearanceDescriptor), With<TileComponent>>,
) -> Option<AppearanceDescriptor> {
    match q_current_appearance.get(
        tiles
            .get(&new_bundle.tile_pos)?
            .peek_for_layer(new_bundle.layer)?,
    ) {
        Ok((visibility, _, appearance)) if visibility != Visibility::Hidden => Some(*appearance),
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
    action_state: Res<ActionState<CompassAction>>,
) {
    for mut cursor in &mut cursor_query {
        if let InputType::SingleClick(size) = cursor.drawing_state.input_type {
            *previous_size = size;
        }

        if action_state.just_pressed(&CompassAction::StartConnectingPoints) {
            cursor.drawing_state.input_type = InputType::DoubleClick(None);
        } else {
            cursor.drawing_state.input_type = InputType::SingleClick(*previous_size);
        }
    }
}
