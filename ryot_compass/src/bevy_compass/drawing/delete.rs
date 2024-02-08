use crate::{Cursor, DrawingAction};
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use ryot::bevy_ryot::*;
use ryot::prelude::{drawing::*, position::*};

/// A function that listens to the right mouse button and deletes the content of the tile under the cursor.
/// It always delete the topmost content of the tile, following the Z-ordering.
pub(super) fn delete_tile_content(
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
            .push(TileCommandRecord::new(layer, tile_pos, Box::new(command)).into());
    }
}
