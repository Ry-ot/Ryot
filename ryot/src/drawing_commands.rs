use crate::bevy_ryot::AppearanceDescriptor;
use crate::drawing::Layer;
use crate::position::TilePosition;
use bevy::ecs::system::{Command, EntityCommand};
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Default, Resource)]
pub struct CommandHistory {
    pub commands: Vec<UndoableCommand>,
}

pub trait ReversibleCommand: Command + Send + Sync + 'static {
    fn undo(&self, commands: &mut Commands);
}

pub trait ReversibleEntityCommand: EntityCommand + Send + Sync + 'static {
    fn undo(&self, entity: Entity, commands: &mut Commands);
}

pub enum UndoableCommand {
    Regular(Box<dyn ReversibleCommand>),
    Entity(Entity, Box<dyn ReversibleEntityCommand>),
}

#[derive(Debug, Default, Resource, Deref, DerefMut)]
pub struct MapTiles(pub HashMap<TilePosition, HashMap<Layer, Entity>>);

#[derive(Debug, Clone)]
pub struct DeleteTileContent(pub TilePosition, pub AppearanceDescriptor, pub Layer);
impl EntityCommand for DeleteTileContent {
    fn apply(self, id: Entity, world: &mut World) {
        let DeleteTileContent(tile_pos, _, layer) = &self;
        world.despawn(id);

        let mut map_tiles = world.resource_mut::<MapTiles>();
        if let Some(content) = map_tiles.0.get_mut(tile_pos) {
            content.remove(layer);
        }
    }
}

impl ReversibleEntityCommand for DeleteTileContent {
    fn undo(&self, _: Entity, commands: &mut Commands) {
        let entity = commands.spawn_empty().id();
        commands.add(AddTileContent(self.0, self.1.clone(), self.2).with_entity(entity));
    }
}

#[derive(Debug, Clone)]
pub struct AddTileContent(pub TilePosition, pub AppearanceDescriptor, pub Layer);
impl EntityCommand for AddTileContent {
    fn apply(self, id: Entity, world: &mut World) {
        let AddTileContent(tile_pos, desired_appearance, layer) = self;
        world
            .entity_mut(id)
            .insert((desired_appearance, tile_pos, layer));

        let mut map_tiles = world.resource_mut::<MapTiles>();
        let content = map_tiles.entry(self.0).or_default();
        content.insert(layer, id);
    }
}

impl ReversibleEntityCommand for AddTileContent {
    fn undo(&self, _: Entity, commands: &mut Commands) {
        commands.add(ChangeTileContentVisibility(
            self.0,
            Visibility::Hidden,
            self.2,
        ));
    }
}

#[derive(Debug, Clone)]
pub struct ChangeTileContentVisibility(pub TilePosition, pub Visibility, pub Layer);

impl Command for ChangeTileContentVisibility {
    fn apply(self, world: &mut World) {
        let ChangeTileContentVisibility(tile_pos, tile_visibility, layer) = self;

        // Separate the entities to modify from the MapTiles resource borrowing scope
        let to_modify = {
            let mut map_tiles = world.resource_mut::<MapTiles>();
            if let Some(content) = map_tiles.0.get_mut(&tile_pos) {
                content.get(&layer).cloned()
            } else {
                None
            }
        };

        // Apply changes to entities outside of the MapTiles borrowing scope
        if let Some(entity) = to_modify {
            if let Some(mut visibility) = world.get_mut::<Visibility>(entity) {
                *visibility = tile_visibility;
            }
        }
    }
}

impl ReversibleCommand for ChangeTileContentVisibility {
    fn undo(&self, commands: &mut Commands) {
        commands.add(ChangeTileContentVisibility(
            self.0,
            match self.1 {
                Visibility::Hidden => Visibility::Visible,
                Visibility::Visible => Visibility::Hidden,
                _ => self.1,
            },
            self.2,
        ));
    }
}

#[derive(Debug, Clone)]
pub struct UpdateTileContent(
    pub Option<AppearanceDescriptor>,
    pub Option<AppearanceDescriptor>,
);
impl EntityCommand for UpdateTileContent {
    fn apply(self, id: Entity, world: &mut World) {
        let UpdateTileContent(loaded_sprite, _) = &self;

        let Some(loaded_sprite) = loaded_sprite else {
            world.despawn(id);
            return;
        };

        if let Some(mut loaded) = world.get_mut::<AppearanceDescriptor>(id) {
            *loaded = loaded_sprite.clone();
        }

        if let Some(mut visibility) = world.get_mut::<Visibility>(id) {
            *visibility = Visibility::Visible;
        }
    }
}

impl ReversibleEntityCommand for UpdateTileContent {
    fn undo(&self, entity: Entity, commands: &mut Commands) {
        commands.add(UpdateTileContent(self.1.clone(), self.0.clone()).with_entity(entity));
    }
}
