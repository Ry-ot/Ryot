use crate::bevy_ryot::drawing::{
    create, DeleteTileContent, Deletion, DrawingBundle, ReversibleCommand,
};
use crate::bevy_ryot::AppearanceDescriptor;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::{Commands, Entity, Visibility, World};

/// A command that updates the content of a tile. An entity location is represented by the
/// combination of a Layer and a Position.
///
/// The command holds the new content (the one to be applied) and the old content (the one
/// that was replaced), both of them being optional. There are 4 possible scenarios:
/// - The new content is Some and the old content is None: The location is empty and the new
///  content will be added to it.
/// - The new content is None and the old content is Some: The location has content and it will
///  be removed. Removal is expensive, so we just make it invisible for now.
/// - The new content is Some and the old content is Some: The location has content and it will
///  be replaced by the new content.
/// - The new content is None and the old content is None: The location is empty and nothing
///  will be done, remaining empty.
///
/// The command updates the whole DrawingBundle of the tile.
///
/// The old content is also used to revert the command, so that the previous state can be
/// restored.
#[derive(Debug, Copy, Clone)]
pub struct UpdateTileContent(pub DrawingBundle, pub Option<DrawingBundle>);
impl EntityCommand for UpdateTileContent {
    fn apply(self, id: Entity, world: &mut World) {
        match self {
            UpdateTileContent(new_bundle, None) => create(id, world, new_bundle),
            UpdateTileContent(new_bundle, Some(_)) => update(id, world, new_bundle),
        }
    }
}

/// Undoing `UpdateTileContent` is done by simply applying a new command with the old content
/// as new content and the new content as old content. This way, the previous state is restored.
/// Redoing the command is done by simply applying the command again, with the provided entity.
impl ReversibleCommand for UpdateTileContent {
    fn undo(&self, commands: &mut Commands, entity: Option<Entity>) {
        if let Some(entity) = entity {
            match self {
                UpdateTileContent(_, None) => commands.add(DeleteTileContent(vec![self.0])),
                UpdateTileContent(_, Some(old)) => {
                    commands.add(UpdateTileContent(*old, Some(self.0)).with_entity(entity))
                }
            }
        }
    }

    fn redo(&self, commands: &mut Commands, entity: Option<Entity>) {
        if let Some(entity) = entity {
            commands.add(self.with_entity(entity));
        }
    }
}

pub fn update(id: Entity, world: &mut World, bundle: DrawingBundle) {
    world.entity_mut(id).remove::<Deletion>();

    let Some(mut descriptor) = world.get_mut::<AppearanceDescriptor>(id) else {
        return;
    };

    if *descriptor != bundle.appearance {
        *descriptor = bundle.appearance;
    }

    if let Some(mut visibility) = world.get_mut::<Visibility>(id) {
        *visibility = bundle.visibility
    }
}
