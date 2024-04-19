use crate::{
    delete_top_most_elements_in_positions, CommandHistory, CompassAction, Cursor, CursorCommand,
    InputType, ToolMode,
};
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use ryot::bevy_ryot::*;
use ryot::prelude::{drawing::*, *};

/// System responsible for handling the drawing inputs. Drawing inputs can be a multitude of things,
/// such as drawing, erasing, selecting, etc. In our context, we are only handling drawing and erasing,
/// keeping the possibility of adding more tools in the future (e.g. if we want special tools for marking
/// some protection zone in the map, for drawing paths, creating areas/cities, etc).
pub fn handle_drawing_input<C: ContentAssets>(
    mut commands: Commands,
    mut tiles: ResMut<MapTiles<Entity>>,
    mut command_history: ResMut<CommandHistory>,
    content_assets: Res<C>,
    brushes: Res<Brushes<DrawingBundle>>,
    q_current: Query<
        (&Visibility, &GameObjectId, Option<&FrameGroup>),
        (With<TileComponent>, With<Layer>),
    >,
    cursor_query: Query<(Option<&GameObjectId>, &TilePosition, &Cursor)>,
) {
    get_cursor_inputs(
        &content_assets,
        &brushes,
        &cursor_query,
        |cursor: &Cursor, bundles: Vec<DrawingBundle>| match cursor.drawing_state.tool_mode {
            Some(ToolMode::Draw(_)) => {
                create_or_update_content_for_positions(
                    &bundles,
                    &mut commands,
                    &mut command_history,
                    &mut tiles,
                    &q_current,
                );
            }
            Some(ToolMode::Erase) => {
                delete_top_most_elements_in_positions(
                    get_top_most_visible_for_bundles(&bundles, &mut tiles, &q_current),
                    &mut commands,
                    &mut command_history,
                );
            }
            _ => (),
        },
    );
}

fn get_cursor_inputs<C: ContentAssets, F: QueryFilter>(
    content_assets: &Res<C>,
    brushes: &Res<Brushes<DrawingBundle>>,
    cursor_query: &Query<(Option<&GameObjectId>, &TilePosition, &Cursor), F>,
    mut callback: impl FnMut(&Cursor, Vec<DrawingBundle>),
) {
    if content_assets.sprite_sheet_data_set().is_none() {
        warn!("Trying to draw a sprite without any loaded content");
        return;
    };

    for (object_id, tile_pos, cursor) in cursor_query {
        let (object_id, layer) = match object_id {
            None => (GameObjectId::default(), Layer::default()),
            Some(object_id) => {
                let Some((group, id)) = object_id.as_group_and_id() else {
                    continue;
                };
                let Some(prepared_appearance) = content_assets
                    .prepared_appearances()
                    .get_for_group(group, id)
                else {
                    continue;
                };

                (*object_id, Layer::from(prepared_appearance.category))
            }
        };

        callback(
            cursor,
            brushes(
                cursor.drawing_state.brush_index,
                cursor.drawing_state.input_type.into(),
                DrawingBundle::new(layer, *tile_pos, object_id, default()),
            ),
        );
    }
}

fn create_or_update_content_for_positions(
    to_draw: &[DrawingBundle],
    commands: &mut Commands,
    command_history: &mut ResMut<CommandHistory>,
    tiles: &mut ResMut<MapTiles<Entity>>,
    q_current: &Query<
        (&Visibility, &GameObjectId, Option<&FrameGroup>),
        (With<TileComponent>, With<Layer>),
    >,
) {
    let mut old_info: Vec<DrawingInfo> = vec![];
    let mut to_draw = to_draw.to_vec();

    for new_bundle in to_draw.iter_mut() {
        match new_bundle.layer {
            Layer::Bottom(_) => {
                let top_most =
                    get_top_most_visible_bottom_layer(new_bundle.tile_pos, tiles, q_current);

                let Some((_, old_bundle)) = top_most else {
                    old_info.push((
                        new_bundle.tile_pos,
                        new_bundle.layer,
                        new_bundle.visibility,
                        None,
                    ));
                    continue;
                };

                if let Layer::Bottom(mut bottom) = old_bundle.layer {
                    new_bundle.layer = Layer::Bottom(bottom.next().unwrap_or(bottom));

                    old_info.push((
                        new_bundle.tile_pos,
                        new_bundle.layer,
                        new_bundle.visibility,
                        if new_bundle.layer != old_bundle.layer {
                            None
                        } else {
                            Some((old_bundle.object_id, old_bundle.frame_group))
                        },
                    ));
                };
            }
            _ => old_info.push((
                new_bundle.tile_pos,
                new_bundle.layer,
                new_bundle.visibility,
                get_current_appearance(*new_bundle, tiles, q_current),
            )),
        }
    }

    let new_info = to_draw
        .iter()
        .copied()
        .map(|bundle| bundle.into())
        .collect::<Vec<DrawingInfo>>();

    if new_info
        .iter()
        .filter_map(|info| info.3?.0.id())
        .eq(old_info.iter().filter_map(|info| info.3?.0.id()))
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
    tiles: &mut ResMut<MapTiles<Entity>>,
    q_current: &Query<
        (&Visibility, &GameObjectId, Option<&FrameGroup>),
        (With<TileComponent>, With<Layer>),
    >,
) -> Option<(GameObjectId, FrameGroup)> {
    match q_current.get(
        tiles
            .get(&new_bundle.tile_pos)?
            .peek_for_layer(new_bundle.layer)?,
    ) {
        Ok((visibility, object_id, frame_group)) if visibility != Visibility::Hidden => {
            Some((*object_id, frame_group.cloned().unwrap_or_default()))
        }
        _ => None,
    }
}

pub fn update_drawing_input_type(
    cursor_query: Query<(&TilePosition, &Cursor)>,
    mut cursor_events_writer: EventWriter<CursorCommand>,
) {
    for (cursor_pos, cursor) in &cursor_query {
        let input_type = match cursor.drawing_state.input_type {
            InputType::DoubleClick(_) => InputType::DoubleClick(Some(*cursor_pos)),
            input_type => input_type,
        };

        if cursor.drawing_state.input_type != input_type {
            cursor_events_writer.send(CursorCommand::ChangeInputType(input_type));
        }
    }
}

pub fn set_drawing_input_type(
    mut previous_size: Local<i32>,
    cursor_query: Query<&Cursor>,
    action_state: Res<ActionState<CompassAction>>,
    mut cursor_events_writer: EventWriter<CursorCommand>,
) {
    for cursor in &cursor_query {
        if let InputType::SingleClick(size) = cursor.drawing_state.input_type {
            *previous_size = size;
        }

        let input_type = if action_state.just_pressed(&CompassAction::StartConnectingPoints) {
            InputType::DoubleClick(None)
        } else {
            InputType::SingleClick(*previous_size)
        };

        if cursor.drawing_state.input_type != input_type {
            cursor_events_writer.send(CursorCommand::ChangeInputType(input_type));
        }
    }
}
