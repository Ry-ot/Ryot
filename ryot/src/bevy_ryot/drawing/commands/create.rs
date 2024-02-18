use crate::bevy_ryot::drawing::{DeleteTileContent, DrawingBundle, ReversibleCommand};
use crate::bevy_ryot::map::MapTiles;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::{Commands, Entity, World};

#[derive(Debug, Copy, Clone)]
pub struct CreateTileContent(pub DrawingBundle);
impl EntityCommand for CreateTileContent {
    fn apply(self, id: Entity, world: &mut World) {
        create(id, world, self.0)
    }
}

impl ReversibleCommand for CreateTileContent {
    fn undo(&self, commands: &mut Commands, entity: Option<Entity>) {
        if let Some(entity) = entity {
            commands.add(DeleteTileContent(vec![self.0]).with_entity(entity));
        }
    }

    fn redo(&self, commands: &mut Commands, entity: Option<Entity>) {
        if let Some(entity) = entity {
            commands.add(self.with_entity(entity));
        }
    }
}

pub fn create(id: Entity, world: &mut World, bundle: DrawingBundle) {
    world.entity_mut(id).insert(bundle);
    let mut map_tiles = world.resource_mut::<MapTiles>();
    let content = map_tiles.entry(bundle.tile_pos).or_default();
    content.insert(bundle.layer, id);
}
