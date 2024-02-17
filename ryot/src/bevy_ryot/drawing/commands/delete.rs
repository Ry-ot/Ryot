use crate::bevy_ryot::drawing::{CreateTileContent, Deleted, DrawingBundle, ReversibleCommand};
use bevy::ecs::system::EntityCommand;
use bevy::prelude::{Commands, Entity, Visibility, World};

#[derive(Debug, Copy, Clone)]
pub struct DeleteTileContent(pub DrawingBundle);
impl EntityCommand for DeleteTileContent {
    fn apply(self, id: Entity, world: &mut World) {
        delete(id, world);
    }
}

impl ReversibleCommand for DeleteTileContent {
    fn undo(&self, commands: &mut Commands, entity: Option<Entity>) {
        if let Some(entity) = entity {
            commands.add(CreateTileContent(self.0).with_entity(entity));
        }
    }

    fn redo(&self, commands: &mut Commands, entity: Option<Entity>) {
        if let Some(entity) = entity {
            commands.add(self.with_entity(entity));
        }
    }
}

pub fn delete(id: Entity, world: &mut World) {
    if let Some(mut visibility) = world.get_mut::<Visibility>(id) {
        *visibility = Visibility::Hidden;
        world.entity_mut(id).insert(Deleted);
    }
}
