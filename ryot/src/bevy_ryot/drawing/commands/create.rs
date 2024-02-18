use crate::bevy_ryot::drawing::{
    CommandType, DeleteTileContent, Deletion, DrawingBundle, ReversibleCommand,
};
use crate::bevy_ryot::map::MapTiles;
use bevy::ecs::system::Command;
use bevy::prelude::{Commands, Entity, World};

#[derive(Debug, Clone)]
pub struct CreateTileContent(pub Vec<DrawingBundle>);

impl From<CreateTileContent> for CommandType {
    fn from(command: CreateTileContent) -> Self {
        CommandType::Command(Box::new(command))
    }
}

impl Command for CreateTileContent {
    fn apply(self, world: &mut World) {
        for bundle in &self.0 {
            let mut map_tiles = world.resource_mut::<MapTiles>();
            let entity = map_tiles
                .entry(bundle.tile_pos)
                .or_default()
                .get(&bundle.layer)
                .copied();

            let id = match entity {
                Some(entity) => entity,
                None => world.spawn_empty().id(),
            };

            world.entity_mut(id).remove::<Deletion>();
            world.entity_mut(id).insert(*bundle);
        }
    }
}

impl ReversibleCommand for CreateTileContent {
    fn undo(&self, commands: &mut Commands, _: Option<Entity>) {
        commands.add(DeleteTileContent(self.clone().0));
    }

    fn redo(&self, commands: &mut Commands, _: Option<Entity>) {
        commands.add(self.clone());
    }
}

pub fn create(id: Entity, world: &mut World, bundle: DrawingBundle) {
    world.entity_mut(id).insert(bundle);
    let mut map_tiles = world.resource_mut::<MapTiles>();
    let content = map_tiles.entry(bundle.tile_pos).or_default();
    content.insert(bundle.layer, id);
}
